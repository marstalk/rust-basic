/**
 * 1. encapsulate by private and pub
 */
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove_head(&mut self) -> Option<i32> {
        let item = self.list.pop();
        if let Some(_) = item {
            self.update_average();
        };
        item
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let mut average_collection = AverageCollection {
            list: vec![1, 2, 3],
            average: 2.0,
        };
        let remove = average_collection.remove_head();
        assert_eq!(remove, Some(3));
        assert_eq!(average_collection.average(), 3 as f64 / 2 as f64);

        assert_eq!(average_collection.remove_head(), Some(2));
        assert_eq!(average_collection.average(), 1.0);
    }

    #[test]
    fn test_add() {
        let mut average_collection = AverageCollection {
            list: vec![],
            average: 0.0,
        };
        average_collection.add(1);
        assert_eq!(average_collection.average(), 1.0);

        average_collection.add(3);
        assert_eq!(average_collection.average(), 2.0);

        average_collection.add(2);
        assert_eq!(average_collection.average(), 2.0);
    }
}
