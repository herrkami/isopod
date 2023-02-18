// Provides a struct that implements a parameterized piecewise linear
// approximation of an exponential curve. Can be used for envelops.

pub struct LinExp<T> {
    sigma: T,
    norm: T,
    sigma_max: T,
}

impl LinExp<i32> {
    pub fn new() -> Self {
        Self {
            sigma: 0,
            norm: (1 << 15),
            sigma_max: (1 << 15) / 2,
        }
    }
    pub fn y(&self, x: i32) -> i32 {
        // x is normalized to [0..norm]
        // Analytically sigma is in [-0.5..0]. Here we map this range to
        // [0..norm/2] and hence sigma has to be normalized by -norm.
        let norm = self.norm;
        return match x {
            // Negative out of bound
            i if i < 0 => norm,
            // A
            // sigma/2 + 1/4
            i if i <= norm / 4 - &self.sigma / 2 => {
                println!("A({}): {} of {}", norm / 4 - &self.sigma / 2, i, norm);
                // x*(4*sigma - 1)/(2*sigma + 1) + 1
                (x * (4 * &self.sigma + norm)) / (2 * &self.sigma - norm) + norm
            }
            // B
            // sigma + 1/2
            i if i <= norm / 2 - &self.sigma => {
                println!("B({}): {} of {}", norm / 2 - &self.sigma, i, norm);
                // x*(- 1)/(2*sigma + 1) + 1 + sigma
                (norm * x) / (2 * &self.sigma - norm) + norm - &self.sigma
            }
            // C
            // sigma + 3/4
            i if i <= (3 * norm) / 4 - &self.sigma => {
                println!("C({}): {} of {}", (3 * norm) / 4 - &self.sigma, i, norm);
                // x*(-2*sigma - 1) + (1 + sigma)*(2*sigma + 1)
                ((x * (2 * &self.sigma - norm)) + ((norm - &self.sigma) * (norm - 2 * &self.sigma)))
                    / norm
            }
            // D
            // 1
            i if i <= norm => {
                println!("D({}): {} of {}", norm, i, norm);
                // x*(-2*sigma - 1)/(1 - 4*sigma) + (1 + 2*sigma)/(1 - 4*sigma)
                ((x * (2 * &self.sigma - norm)) + (norm * (norm - 2 * &self.sigma)))
                    / (norm + 4 * &self.sigma)
            }
            // Positive out of bound
            _ => 0,
        };
    }
    pub fn set_sigma(&mut self, sigma: i32) {
        if sigma >= 0 && sigma < self.sigma_max {
            self.sigma = sigma;
        } else {
            // TODO should be an error handle here if performance allows it.
            self.sigma = 0;
        }
    }
    pub fn get_sigma_max(&self) -> i32 {
        self.sigma_max
    }
    pub fn get_norm(&self) -> i32 {
        self.norm
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linexp() {
        let mut linexp = LinExp::new();
        let sigma_max = linexp.get_sigma_max();
        let norm = linexp.get_norm();
        linexp.set_sigma(sigma_max - sigma_max / 16);
        for x in (0..norm + 1).step_by(norm as usize / 16) {
            print!("{:?}\n", linexp.y(x));
        }
    }
}
