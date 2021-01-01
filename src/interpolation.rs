use dasp::{interpolate::sinc::Sinc, ring_buffer, signal, Signal};
use ringbuf::{Consumer, Producer, RingBuffer};

struct AudioSrc {
    consumer: Consumer<f32>,
}

impl AudioSrc {
    fn new(capacity: usize) -> (AudioSrc, Producer<f32>) {
        let ring = RingBuffer::<f32>::new(capacity);
        let (producer, consumer) = ring.split();
        (AudioSrc { consumer }, producer)
    }
}

impl Signal for AudioSrc {
    type Frame = f32;

    fn next(&mut self) -> Self::Frame {
        self.consumer.pop().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl() {
        let source_fs = 44100.0;
        let target_fs = 88200.0;

        let sinc = Sinc::new(ring_buffer::Fixed::from([0.0f32; SINC_INTERPOLATOR_SIZE]));

        let (source, mut producer) = AudioSrc::new(512);

        let mut signal = source.from_hz_to_hz(sinc, source_fs, target_fs);

        for n in [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0].iter() {
            producer.push(*n).unwrap();
        }

        for _ in 0..16 {
            print!(" {} ", signal.next());
        }

        for n in [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0].iter() {
            producer.push(*n).unwrap();
        }

        for _ in 0..16 {
            print!(" {} ", signal.next());
        }
    }

    //TODO make generic for any size
    const SINC_INTERPOLATOR_SIZE: usize = 8;

    #[test]
    fn test_gen() {
        let source_fs = 44100.0;
        let target_fs = 88200.0;

        let sinc = Sinc::new(ring_buffer::Fixed::from([0.0f32; SINC_INTERPOLATOR_SIZE]));

        let ring = RingBuffer::<f32>::new(512);

        let (mut producer, mut consumer) = ring.split();

        let source = signal::gen_mut(|| consumer.pop().unwrap_or(0.0));

        let mut signal = source.from_hz_to_hz(sinc, source_fs, target_fs);

        for n in [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0].iter() {
            producer.push(*n).unwrap();
        }

        for _ in 0..16 {
            print!(" {} ", signal.next());
        }

        for n in [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0].iter() {
            producer.push(*n).unwrap();
        }

        for _ in 0..16 {
            print!(" {} ", signal.next());
        }
    }
}
