use crate::core::vector::Vector;
use crate::error::VectorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Euclidean,
    Manhattan,
    CosineSim,
}

impl Distance {
    /// Calculate the distance between two vectors using this metric
    pub fn distance(&self, v1: &Vector, v2: &Vector) -> Result<f32, VectorError> {
        if v1.size() != v2.size() {
            return Err(VectorError::DimensionsMismatch { expected: v1.size(), found: v2.size() });
        }
        
        match self {
            Distance::Euclidean => {
                Ok(v1.data()
                    .iter()
                    .zip(v2.data())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f32>()
                    .sqrt())
            }
            Distance::Manhattan => {
                Ok(v1.data()
                    .iter()
                    .zip(v2.data())
                    .map(|(a, b)| (a - b).abs())
                    .sum())
            }
            Distance::CosineSim => {
                let dot = v1.dot_product(v2)?;
                let norm1 = v1.norm();
                let norm2 = v2.norm();

                if norm1 == 0.0 || norm2 == 0.0 {
                    return Ok(1.0);
                }

                let cosine_similarity = (dot / (norm1 * norm2)).clamp(-1.0, 1.0);
                Ok(1.0 - cosine_similarity)
            }
        }
    }

    /// Get the name of this distance metric as a string
    pub fn name(&self) -> &'static str {
        match self {
            Distance::Euclidean => "euclidean",
            Distance::Manhattan => "manhattan",
            Distance::CosineSim => "cosinesim",
        }
    }

    /// Create a distance metric from a string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "euclidean" | "e" => Some(Distance::Euclidean),
            "manhattan" | "m" => Some(Distance::Manhattan),
            "cosinesim" | "c" => Some(Distance::CosineSim),
            _ => None,
        }
    }
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for Distance {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Distance::from_name(s)
            .ok_or_else(|| format!("Unknown distance metric: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance_calculation() {
        let d = Distance::Euclidean;
        let v1 = Vector::from_slice(&[0.0, 0.0]);
        let v2 = Vector::from_slice(&[3.0, 4.0]);
        assert_eq!(d.distance(&v1, &v2).unwrap(), 5.0);
    }

    #[test]
    fn test_manhattan_distance_calculation() {
        let d = Distance::Manhattan;
        let v1 = Vector::from_slice(&[0.0, 0.0]);
        let v2 = Vector::from_slice(&[3.0, 4.0]);
        assert_eq!(d.distance(&v1, &v2).unwrap(), 7.0);
    }

    #[test]
    fn test_cosine_similarity_distance_calculation() {
        let d = Distance::CosineSim;
        let v1 = Vector::from_slice(&[1.0, 0.0]);
        let v2 = Vector::from_slice(&[1.0, 0.0]);
        // Same vectors should have distance 0 (similarity 1)
        assert!((d.distance(&v1, &v2).unwrap() - 0.0).abs() < 1e-6);

        let v3 = Vector::from_slice(&[-1.0, 0.0]);
        // Opposite vectors should have distance 2 (similarity -1)
        assert!((d.distance(&v1, &v3).unwrap() - 2.0).abs() < 1e-6);

        let v4 = Vector::from_slice(&[0.0, 1.0]);
        // Perpendicular vectors should have distance 1 (similarity 0)
        assert!((d.distance(&v1, &v4).unwrap() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_distance_dimension_mismatch_errors() {
        let d = Distance::Euclidean;
        let v1 = Vector::from_slice(&[1.0, 2.0]);
        let v2 = Vector::from_slice(&[1.0, 2.0, 3.0]);
        let result = d.distance(&v1, &v2);
        assert!(result.is_err());
    }
} 