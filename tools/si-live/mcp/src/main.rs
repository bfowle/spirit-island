//! si-mcp — Minimal MCP stdio server for Claude Code.
//!
//! Scope for Phase A: stub that reads `data/current-game.json` and exposes
//! a `get_game_state` tool. Will expand to full MCP protocol (tools +
//! resources) in Phase B.
//!
//! Communication: JSON-RPC 2.0 over stdin/stdout, per the MCP spec.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use si_common::GameState;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Debug, Deserialize)]
struct Request {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct Response {
    jsonrpc: &'static str,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ResponseError>,
}

#[derive(Debug, Serialize)]
struct ResponseError {
    code: i32,
    message: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state_path: PathBuf = std::env::var("SI_STATE_PATH")
        .unwrap_or_else(|_| "data/current-game.json".to_string())
        .into();

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut lines = BufReader::new(stdin).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let req: Request = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                send_error(&mut stdout, None, -32700, format!("parse error: {e}")).await?;
                continue;
            }
        };
        if req.jsonrpc != "2.0" {
            send_error(&mut stdout, req.id, -32600, "jsonrpc must be 2.0".into()).await?;
            continue;
        }
        let response = match req.method.as_str() {
            "initialize" => Response {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(serde_json::json!({
                    "serverInfo": {"name": "si-mcp", "version": env!("CARGO_PKG_VERSION")},
                    "capabilities": {
                        "tools": {},
                        "resources": {},
                    },
                })),
                error: None,
            },
            "tools/list" => Response {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(serde_json::json!({
                    "tools": [
                        {
                            "name": "get_game_state",
                            "description": "Return the current Spirit Island game state",
                            "inputSchema": {"type": "object", "properties": {}, "required": []},
                        }
                    ]
                })),
                error: None,
            },
            "tools/call" => {
                let name = req.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                match name {
                    "get_game_state" => match load_state(&state_path) {
                        Ok(state) => Response {
                            jsonrpc: "2.0",
                            id: req.id,
                            result: Some(serde_json::json!({
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&state).unwrap_or_default(),
                                }]
                            })),
                            error: None,
                        },
                        Err(e) => Response {
                            jsonrpc: "2.0",
                            id: req.id,
                            result: None,
                            error: Some(ResponseError {
                                code: -32000,
                                message: format!("state read error: {e}"),
                            }),
                        },
                    },
                    other => Response {
                        jsonrpc: "2.0",
                        id: req.id,
                        result: None,
                        error: Some(ResponseError {
                            code: -32601,
                            message: format!("unknown tool: {other}"),
                        }),
                    },
                }
            }
            other => Response {
                jsonrpc: "2.0",
                id: req.id,
                result: None,
                error: Some(ResponseError {
                    code: -32601,
                    message: format!("method not found: {other}"),
                }),
            },
        };
        send(&mut stdout, &response).await?;
    }
    Ok(())
}

fn load_state(path: &PathBuf) -> Result<GameState> {
    let raw = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

async fn send(out: &mut tokio::io::Stdout, resp: &Response) -> Result<()> {
    let line = serde_json::to_string(resp)?;
    out.write_all(line.as_bytes()).await?;
    out.write_all(b"\n").await?;
    out.flush().await?;
    Ok(())
}

async fn send_error(
    out: &mut tokio::io::Stdout,
    id: Option<Value>,
    code: i32,
    message: String,
) -> Result<()> {
    let resp = Response {
        jsonrpc: "2.0",
        id,
        result: None,
        error: Some(ResponseError { code, message }),
    };
    send(out, &resp).await
}
