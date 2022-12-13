use anyhow::Result;

pub fn parser_rgb_str(s: &str) -> Result<(f64, f64, f64)> {
    let s = s
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("rgb(", "")
        .replace(")", "");
    let mut s = s.split(",");
    let r = s.next().unwrap().parse::<f64>()?;
    let g = s.next().unwrap().parse::<f64>()?;
    let b = s.next().unwrap().parse::<f64>()?;
    if r < 0.0 || r > 255.0 {
        return Err(anyhow::anyhow!("r must be between 0 and 255, got {}", r));
    }
    if g < 0.0 || g > 255.0 {
        return Err(anyhow::anyhow!("g must be between 0 and 255, got {}", g));
    }
    if b < 0.0 || b > 255.0 {
        return Err(anyhow::anyhow!("b must be between 0 and 255, got {}", b));
    }
    Ok((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_rgb() {
        let s = "rgb(255, 255, 255)";
        let (r, g, b) = parser_rgb_str(s).unwrap();
        assert_eq!((r, g, b), (255.0, 255.0, 255.0));

        let s = "rgb(0, 0, 0)";
        let (r, g, b) = parser_rgb_str(s).unwrap();
        assert_eq!((r, g, b), (0.0, 0.0, 0.0));

        let s = "rgb255, 0, 0)";
        let s = parser_rgb_str(s);
        assert!(s.is_err());

        let s = "rgb(2555, 0, 0)";
        let s = parser_rgb_str(s);
        assert!(s.is_err());
    }
}
