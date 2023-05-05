use criterion::measurement::ValueFormatter;
use criterion::Throughput;

pub struct EnergyFormatter;

impl ValueFormatter for EnergyFormatter {
    fn format_value(&self, joules: f64) -> String {
        if joules < 1e-9 {
            format!("{:>6} nJ", joules * 1e9)
        } else if joules < 1e-6 {
            format!("{:>6} uJ", joules * 1e6)
        } else if joules < 1e-3 {
            format!("{:>6} mJ", joules * 1e3)
        } else if joules < 1e6 {
            format!("{:>6} J", joules)
        } else {
            format!("{:>6} kJ", joules * 1e-3)
        }
    }

    fn format_throughput(&self, throughput: &Throughput, joules: f64) -> String {
        match *throughput {
            Throughput::Bytes(bytes) => format!(
                "{} B/mJ",
                f64::from(bytes as u32) / joules
            ),
            Throughput::Elements(elems) => format!(
                "{} elem/mJ",
                f64::from(elems as u32) / joules
            ),
            #[cfg(feature = "criterion4")]
            Throughput::BytesDecimal(bytes) => format!(
                "{} B/mJ",
                (bytes as f64) / joules
            )
        }
    }

    fn scale_values(&self, _typical_value: f64, values: &mut [f64]) -> &'static str {
        "J"
    }

    fn scale_throughputs(&self, _typical_value: f64, throughput: &Throughput, values: &mut [f64]) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) => {
                // Convert mJ/iteration to bytes/mJ.
                for val in values {
                    *val = (bytes as f64) / (*val)
                }
                "B/mJ"
            }
            Throughput::Elements(elems) => {
                // Convert mJ/iteration to elems/mJ.
                for val in values {
                    *val = (elems as f64) / (*val)
                }
                "elem/mJ"
            }
            #[cfg(feature = "criterion4")]
            Throughput::BytesDecimal(bytes) => {
                for val in values {
                    *val = (bytes as f64) / (*val)
                }
                "B/mJ"
            }
        }
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        // for val in values {
        //     *val /= 1e3;
        // }
        "J"
    }
}