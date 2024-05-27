use std::{cmp::Ordering, fmt::Display, ops::DerefMut};

use crate::Object;
use num_traits::Zero;
use ordered_float::NotNan;

pub mod err {
    #[derive(thiserror::Error, Debug)]
    #[error("Not enough space for the pushed object.")]
    pub struct TooLittleSpace;
}

pub use progress::FillingProgress;
pub mod progress {
    use indicatif::ProgressBar;
    use num_bigint::BigUint;
    use num_rational::Ratio;
    use num_traits::Zero;

    pub struct FillingProgress {
        progress: BigUint,
        observed_progress: u64,
        scale: Ratio<BigUint>,
        bar: ProgressBar,
    }
    impl FillingProgress {
        pub fn new(comb_count: BigUint) -> Self {
            let observable_step_count = u64::MAX;
            let scale = Ratio::new(observable_step_count.into(), comb_count);
            Self {
                progress: Zero::zero(),
                observed_progress: Zero::zero(),
                scale,
                bar: ProgressBar::new(observable_step_count),
            }
        }
        pub fn lg_inc(&mut self, delta_lg: u32) {
            let delta = BigUint::from(2u8).pow(delta_lg);
            self.inc(delta);
        }
        pub fn inc(&mut self, delta: BigUint) {
            self.progress += &delta;
            let new_observed_progress = &self.scale * &self.progress;
            let new_observed_progress: u64 =
                new_observed_progress.round().numer().try_into().unwrap();
            if new_observed_progress > self.observed_progress {
                self.observed_progress = new_observed_progress;
                self.bar.inc(new_observed_progress - self.observed_progress);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Knapsack {
    items: Vec<Object>,
    value: NotNan<f64>,
    used_space: NotNan<f64>,
    capacity: NotNan<f64>,
}
impl Knapsack {
    // CRUD-C: Constructors
    pub fn new(capacity: NotNan<f64>) -> Self {
        Self {
            items: Default::default(),
            value: Zero::zero(),
            used_space: Zero::zero(),
            capacity,
        }
    }
    pub fn filled_with_most_valuable<II, I>(
        capacity: NotNan<f64>,
        objs_for_packing: II,
        #[cfg(feature = "progress_bar")] progress_bar: Option<&mut FillingProgress>,
    ) -> Knapsack
    where
        II: IntoIterator<IntoIter = I>,
        I: Iterator<Item = Object> + Clone,
    {
        let mut new_instance = Self::new(capacity);
        new_instance.fill_with_most_valuable(
            objs_for_packing,
            #[cfg(feature = "progress_bar")]
            progress_bar,
        );
        new_instance
    }
    // CRUD-R: Getters
    pub fn items<'s>(&'s self) -> impl Iterator<Item = Object> + 's {
        self.items.iter().copied()
    }
    pub fn value(&self) -> NotNan<f64> {
        self.value
    }
    pub fn used_space(&self) -> NotNan<f64> {
        self.used_space
    }
    pub fn capacity(&self) -> NotNan<f64> {
        self.capacity
    }
    // CRUD-R: Properties
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn used_space_after_push(&self, obj: &Object) -> Result<NotNan<f64>, err::TooLittleSpace> {
        let new_used_space = self.used_space + obj.volume();
        if new_used_space > self.capacity {
            return Err(err::TooLittleSpace);
        }
        Ok(new_used_space)
    }
    pub fn can_push(&self, obj: &Object) -> bool {
        self.used_space_after_push(obj).is_ok()
    }
    pub fn properties(&self) -> Properties {
        Properties {
            value: self.value(),
            item_count: self.item_count(),
            used_space: self.used_space(),
            capacity: self.capacity(),
        }
    }
    // CRUD-R: Comparators
    pub fn cmp_value(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
    // CRUD-U: Update
    pub fn fill_with_most_valuable<II, I>(
        &mut self,
        objs_left: II,
        #[cfg(feature = "progress_bar")] mut progress_bar: Option<&mut FillingProgress>,
    ) where
        II: IntoIterator<IntoIter = I>,
        I: Iterator<Item = Object> + Clone,
    {
        let mut objs_left = objs_left.into_iter();
        let Some(next_obj) = objs_left.next() else {
            return;
        };
        let greedy_alter_ego = if self.can_push(&next_obj) {
            let mut greedy_alter_ego = self.clone();
            greedy_alter_ego.push(next_obj);
            greedy_alter_ego.fill_with_most_valuable(
                objs_left.clone(),
                #[cfg(feature = "progress_bar")]
                progress_bar.as_mut().map(DerefMut::deref_mut),
            );
            Some(greedy_alter_ego)
        } else {
            None
        };
        self.fill_with_most_valuable(
            objs_left,
            #[cfg(feature = "progress_bar")]
            progress_bar,
        );
        greedy_alter_ego
            .map(|alter_ego| {
                if alter_ego.value() > self.value() {
                    *self = alter_ego;
                };
            })
            .unwrap_or_default()
    }
    pub fn push(&mut self, next_obj: Object) {
        self.try_push(next_obj)
            .expect("Should be called in \"can push\" branch.");
    }
    pub fn try_push(&mut self, obj: Object) -> Result<(), err::TooLittleSpace> {
        self.used_space = self.used_space_after_push(&obj)?;
        self.value += obj.value();
        Ok(self.items.push(obj))
    }
}

// CRUD-R: Displayers
impl Display for Knapsack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use tabled::Table;
        let properties = Table::new([self.properties()]);
        let items = Table::new(self.items());
        write!(
            f,
            "Knapsack properties:\n{properties}\nKnapsack items:\n{items}"
        )
    }
}

pub use props::Properties;
pub mod props {
    use ordered_float::NotNan;

    #[derive(Clone, PartialEq, Eq, Debug, tabled::Tabled)]
    pub struct Properties {
        pub value: NotNan<f64>,
        pub item_count: usize,
        pub used_space: NotNan<f64>,
        pub capacity: NotNan<f64>,
    }
}
