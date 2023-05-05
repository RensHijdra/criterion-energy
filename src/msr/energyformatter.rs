use criterion::measurement::ValueFormatter;
use criterion::Throughput;

pub struct EnergyFormatter;

impl ValueFormatter for EnergyFormatter {
    fn format_value(&self, uj: f64) -> String {
        if uj < 1.0 {  // uj = energy in microjoules per iteration
            format!("{:>6} nJ", uj * 1e3)
        } else if uj < 10f64.powi(3) {
            format!("{:>6} uJ", uj)
        } else if uj < 10f64.powi(6) {
            format!("{:>6} mJ", uj / 1e3)
        } else if uj < 10f64.powi(9) {
            format!("{:>6} J", uj / 1e6)
        } else {
            format!("{:>6} KJ", uj / 1e9)
        }
    }

    fn format_throughput(&self, throughput: &Throughput, uj: f64) -> String {
        match *throughput {
            Throughput::Bytes(bytes) => format!(
                "{} B/uJ",
                f64::from(bytes as u32) / uj
            ),
            Throughput::Elements(elems) => format!(
                "{} elem/s/2",
                f64::from(elems as u32) / uj
            ),
            #[cfg(feature = "criterion4")]
            Throughput::BytesDecimal(bytes) => format!(
                "{} B/uJ",
                (bytes as f64) / uj
            )
        }
    }

    fn scale_values(&self, _typical_value: f64, _values: &mut [f64]) -> &'static str {
        todo!()
    }

    fn scale_throughputs(&self, _typical_value: f64, throughput: &Throughput, values: &mut [f64]) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) => {
                // Convert nanoseconds/iteration to bytes/half-second.
                for val in values {
                    *val = (bytes as f64) / (*val)
                }

                "B/uJ"
            }
            Throughput::Elements(elems) => {
                for val in values {
                    *val = (elems as f64) / (*val)
                }
                "elem/uJ"
            },
            #[cfg(feature = "criterion4")]
            Throughput::BytesDecimal(bytes) => {
                for val in values {
                    *val = (bytes as f64) / (*val)
                }
                "B/uJ"
            }
        }

    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        todo!()
    }
}