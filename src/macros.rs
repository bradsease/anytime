macro_rules! impl_from_anytime {
    ($scale:ty) => {
        use crate::AnyTime;
        impl From<AnyTime> for Time<$scale> {
            fn from(any_time: AnyTime) -> Self {
                match any_time {
                    AnyTime::GPST(t) => t.into(),
                    AnyTime::TAI(t) => t.into(),
                    AnyTime::TCB(t) => t.into(),
                    AnyTime::TCG(t) => t.into(),
                    AnyTime::TDB(t) => t.into(),
                    AnyTime::TT(t) => t.into(),
                    AnyTime::UT1(t) => t.into(),
                    AnyTime::UTC(t) => t.into(),
                }
            }
        }
    };
}

pub(crate) use impl_from_anytime;

macro_rules! impl_to_scale {
    ($scale:ty, $method:ident) => {
        impl<S> Time<S>
        where
            Time<S>: Into<Time<$scale>>,
        {
            #[doc = concat!("Converts this time to the `", stringify!($method), "` time scale.")]
            pub fn $method(self) -> Time<$scale> {
                self.into()
            }
        }

        impl AnyTime {
            #[doc = concat!("Converts this value to the `", stringify!($method), "` time scale.")]
            pub fn $method(self) -> Time<$scale> {
                self.into()
            }
        }
    };
}

pub(crate) use impl_to_scale;

macro_rules! impl_time_series_from {
    ($source:ty => $target:ty) => {
        impl From<$crate::TimeSeries<$source>> for $crate::TimeSeries<$target> {
            fn from(series: $crate::TimeSeries<$source>) -> Self {
                Self::new(series.into_iter().map(Into::into).collect())
            }
        }
    };
}

pub(crate) use impl_time_series_from;
