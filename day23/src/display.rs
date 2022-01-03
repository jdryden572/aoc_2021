use std::fmt::Display;

use crate::Burrow;

impl Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec!['.'; 19];
        for (i, &c) in ['A', 'A', 'B', 'B', 'C', 'C', 'D', 'D'].iter().enumerate() {
            let pos = self.pods[i] as usize;
            chars[pos] = c;
        }

        writeln!(f, "{}", String::from_iter(&chars[0..11]))?;
        writeln!(
            f,
            "  {} {} {} {}",
            &chars[11], &chars[12], &chars[13], &chars[14]
        )?;
        writeln!(
            f,
            "  {} {} {} {}",
            &chars[15], &chars[16], &chars[17], &chars[18]
        )?;

        Ok(())
    }
}
