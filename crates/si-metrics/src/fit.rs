//! Small least-squares helpers: ordinary least squares by normal equations
//! (with a whisper of ridge so collinear columns do not blow up), and the
//! fit of a short series to a growth order.

/// Solve min ‖Xβ − y‖² for β. `x[i]` is row i (include a 1.0 for the
/// intercept). None when the system is empty.
pub fn least_squares(x: &[Vec<f64>], y: &[f64]) -> Option<Vec<f64>> {
    let n = x.len();
    let p = x.first()?.len();
    if n == 0 || y.len() != n {
        return None;
    }
    // normal equations
    let mut a = vec![vec![0.0; p + 1]; p];
    for (row, &yi) in x.iter().zip(y) {
        for i in 0..p {
            for j in 0..p {
                a[i][j] += row[i] * row[j];
            }
            a[i][p] += row[i] * yi;
        }
    }
    for i in 0..p {
        a[i][i] += 1e-9;
    }
    // Gaussian elimination with partial pivoting
    for col in 0..p {
        let pivot = (col..p).max_by(|&i, &j| a[i][col].abs().partial_cmp(&a[j][col].abs()).unwrap())?;
        a.swap(col, pivot);
        let d = a[col][col];
        if d.abs() < 1e-12 {
            continue;
        }
        for j in col..=p {
            a[col][j] /= d;
        }
        for i in 0..p {
            if i != col {
                let f = a[i][col];
                if f != 0.0 {
                    for j in col..=p {
                        a[i][j] -= f * a[col][j];
                    }
                }
            }
        }
    }
    Some((0..p).map(|i| a[i][p]).collect())
}

/// Residuals y − Xβ.
pub fn residuals(x: &[Vec<f64>], y: &[f64], beta: &[f64]) -> Vec<f64> {
    x.iter().zip(y).map(|(row, yi)| yi - row.iter().zip(beta).map(|(a, b)| a * b).sum::<f64>()).collect()
}

/// Coefficient of determination.
pub fn r_squared(y: &[f64], res: &[f64]) -> f64 {
    let mean = y.iter().sum::<f64>() / y.len().max(1) as f64;
    let ss_tot: f64 = y.iter().map(|v| (v - mean).powi(2)).sum();
    let ss_res: f64 = res.iter().map(|r| r * r).sum();
    if ss_tot == 0.0 { 1.0 } else { 1.0 - ss_res / ss_tot }
}
