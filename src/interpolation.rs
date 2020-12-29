use dasp::{interpolate::linear::Linear, interpolate::sinc::Sinc, ring_buffer, signal, Signal};

pub struct LinearInterpolator {
    prev_a: f32,
    prev_b: f32,
}

impl LinearInterpolator {
    pub fn new() -> LinearInterpolator {
        LinearInterpolator {
            prev_a: 0.0,
            prev_b: 0.0,
        }
    }

    pub fn process(&mut self, data: &[f32], data_out: &mut [f32], source_hz: f64, target_hz: f64) {
        let source = signal::from_iter(data.iter().cloned());
        let interp = Linear::new(self.prev_a, self.prev_b);
        self.prev_a = data[data.len() - 2];
        self.prev_b = data[data.len() - 1];
        let signal = source.from_hz_to_hz(interp, source_hz, target_hz);
        for (i, x) in signal.until_exhausted().enumerate() {
            if i < 2 {
                continue;
            }
            if i < data_out.len() + 2 {
                data_out[i - 2] = x;
            } else {
                break;
            }
        }
    }
}

//TODO make generic for any size
const SINC_INTERPOLATOR_SIZE: usize = 8;

pub struct SincInterpolator {
    ringbuf: ring_buffer::Fixed<[f32; SINC_INTERPOLATOR_SIZE]>,
}

impl SincInterpolator {
    pub fn new() -> SincInterpolator {
        SincInterpolator {
            ringbuf: ring_buffer::Fixed::from([0.0f32; SINC_INTERPOLATOR_SIZE]),
        }
    }

    ///the size of data_out must be: data * (target_hz/source_hz)
    pub fn process(&mut self, data: &[f32], data_out: &mut [f32], source_hz: f64, target_hz: f64) {
        let source = signal::from_iter(data.iter().cloned());
        let sinc = Sinc::new(self.ringbuf);
        let mut temp = [0.0f32; SINC_INTERPOLATOR_SIZE];
        for (temp, x) in temp
            .iter_mut()
            .zip(data.iter().rev().take(SINC_INTERPOLATOR_SIZE).rev())
        {
            *temp = *x;
        }
        self.ringbuf = ring_buffer::Fixed::from(temp);
        let signal = source.from_hz_to_hz(sinc, source_hz, target_hz);
        for (i, x) in signal.until_exhausted().enumerate() {
            if i < 2 {
                continue;
            }
            if i < data_out.len() + 2 {
                data_out[i - 2] = x;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut resample = SincInterpolator::new();

        let data = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        //the size of data_out must be: data * (target_hz/source_hz)
        let mut data_out = [0.0; 32];
        resample.process(&data, &mut data_out, 1.0, 2.0);

        assert!(
            data_out
                == [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0000000000000000060230073,
                    0.031329945,
                    2.0,
                    1.4900724,
                    3.0,
                    2.501185,
                    4.0,
                    3.5016584,
                    5.0,
                    4.5021334,
                    6.0,
                    5.502607,
                    7.0,
                    6.5030804,
                    8.0,
                    7.5035553,
                    9.0,
                    8.504029,
                    10.0,
                    9.504502,
                    11.0,
                    10.504976,
                    12.0,
                    11.505448,
                    13.0,
                    12.505923,
                    14.0,
                    13.506398
                ]
        );

        let data = [17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];
        let mut data_out = [0.0; 16];

        resample.process(&data, &mut data_out, 1.0, 2.0);

        assert!(
            data_out
                == [
                    12.0, 11.557786, 14.0, 13.482533, 16.0, 15.507347, 18.0, 17.508293, 19.0,
                    18.508764, 20.0, 19.509243, 21.0, 20.509716, 22.0, 21.510185,
                ]
        );
    }
}
