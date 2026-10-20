# Direction review, 2026-10-19

Brett: "the direction is lost. The concepts are being eroded and overwritten. This needs to be deeply thought out."

Five independent reviews, each reading the primary sources (Brett's own messages across both sessions, the specs, the research report, the playtests, the code, the screenshots), then one cohesion pass. The result is the design of record: `docs/superpowers/specs/2026-10-19-direction.md`.

| File | Role |
|---|---|
| `A-intent-and-erosion.md` | the founding intent in Brett's words, the principle contract, the timeline of resets, where the work eroded each principle |
| `F-gameplay-model-referee.md` | layer 1: the game as the Rust code conceives it, coverage matrix, fitness per question, ordered changes |
| `B-models-referee.md` | layer 2: the model stack vs what is built, competing risks as the backbone (exact math), misframings to retire, verification |
| `C-information-design-referee.md` | layer 3: verdict on the built screens, information architecture, visual grammar, wireframes, drift test |
| `E-code-state-audit.md` | the uncommitted tree: keep / keep with changes / discard; the commit split |
| `what-was-built-2026-10-19.md` | the assistant's own account of the two rebuilds and Brett's reactions (checked by A against the record) |
| `sly-inspiration-6..9.png` | the four Sly debugger screenshots (github.com/ioncodes/sly) Brett attached to the reset request as the look he wants |

Brett's verbatim messages were extracted to the session scratchpad for the reviewers and are not in the repository.
