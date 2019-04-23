use super::MaybeUniform;
use crate::distributions::kde::Point;
use crate::distributions::{Pdf, StandardNormal};
use crate::matrix::{Matrix2, Transpose};
use std::f64::consts::PI;

pub trait Kernel<P: Point, RHS = P>: Pdf<P> {
    fn density(&self, x: &P, xi: &RHS, bandwidth: &P::Bandwidth) -> f64;
}
impl Kernel<f64> for StandardNormal {
    fn density(&self, &x: &f64, &xi: &f64, &h: &f64) -> f64 {
        let a = (2.0 * PI).sqrt() * h;
        let b = -(x - xi).powi(2) / (2.0 * h * h);
        b.exp() / a
    }
}
// TODO: delete
impl Pdf<MaybeUniform<f64>> for StandardNormal {
    fn pdf(&self, x: &MaybeUniform<f64>) -> f64 {
        match x {
            MaybeUniform::Sample(x) => StandardNormal.pdf(x),
            MaybeUniform::Uniform(range) => 1.0 / range.width(),
        }
    }
}
impl Kernel<f64, MaybeUniform<f64>> for StandardNormal {
    // TODO
    fn density(&self, x: &f64, xi: &MaybeUniform<f64>, &h: &f64) -> f64 {
        match xi {
            MaybeUniform::Sample(xi) => {
                let a = (2.0 * PI).sqrt() * h;
                let b = -(x - xi).powi(2) / (2.0 * h * h);
                b.exp() / a
            }
            MaybeUniform::Uniform(range) => 1.0 / (range.width() * h),
        }
    }
}
impl Kernel<MaybeUniform<f64>> for StandardNormal {
    // TODO
    fn density(&self, x: &MaybeUniform<f64>, xi: &MaybeUniform<f64>, &h: &f64) -> f64 {
        match (x, xi) {
            (MaybeUniform::Sample(x), MaybeUniform::Sample(xi)) => {
                let a = (2.0 * PI).sqrt() * h;
                let b = -(x - xi).powi(2) / (2.0 * h * h);
                b.exp() / a
            }
            (MaybeUniform::Uniform(_), _) => panic!(),
            (_, MaybeUniform::Uniform(range)) => 1.0 / (range.width() * h),
        }
    }
}
impl Kernel<(f64, f64)> for StandardNormal {
    fn density(&self, &x: &(f64, f64), &xi: &(f64, f64), h: &Matrix2) -> f64 {
        let x = (x.0 - xi.0, x.1 - xi.1);

        let a = 1.0 / (2.0 * PI);
        let b = h.det().powf(-0.5);
        let c = Transpose(x) * (h.inverse() * x);
        a * b * (-0.5 * c).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::Pdf;

    #[test]
    fn normal_pdf_works() {
        let data = [
            (
                -0.05015484986718377,
                4.631375826882106,
                3.4957736565877873e-06,
            ),
            (1.2847681232524142, 0.3286109201987051, 0.06606050594956268),
            (
                0.27153515732684674,
                -2.315645640123003,
                0.010505565626836763,
            ),
            (
                -0.27239977872185506,
                -3.250060892514204,
                0.0007798368797643355,
            ),
            (
                -4.303898993114087,
                1.6030807546462125,
                4.182571576794482e-06,
            ),
            (
                -0.06626695931503779,
                -4.131631437662127,
                3.11947143394824e-05,
            ),
            (1.3865878347895464, 0.22669870948548798, 0.05931541065253321),
            (
                3.5327025938459204,
                2.5624858616925934,
                1.1640295022816219e-05,
            ),
            (
                -1.2145669102041778,
                -4.150551578012804,
                1.3825405819376266e-05,
            ),
            (
                0.04283185697010605,
                -4.575316215934372,
                4.526536011799495e-06,
            ),
        ];
        for (x, y, pdf) in data.iter().cloned() {
            assert_eq!(StandardNormal.pdf(&(x, y)), pdf);
        }
    }

    #[test]
    fn normal_kernel_works() {
        let h = Matrix2::new((2.0, 0.3), (0.3, 0.5));
        let data = [
            (
                -0.05015484986718377,
                4.631375826882106,
                8.953059406467044e-12,
            ),
            (1.2847681232524142, 0.3286109201987051, 0.10821090618543903),
            (
                0.27153515732684674,
                -2.315645640123003,
                0.0003667569844314917,
            ),
            (
                -0.27239977872185506,
                -3.250060892514204,
                1.9911021543184804e-06,
            ),
            (
                -4.303898993114087,
                1.6030807546462125,
                6.279636452680284e-06,
            ),
            (
                -0.06626695931503779,
                -4.131631437662127,
                1.300749775055857e-09,
            ),
            (1.3865878347895464, 0.22669870948548798, 0.10313021247790838),
            (
                3.5327025938459204,
                2.5624858616925934,
                7.863447318780707e-05,
            ),
            (
                -1.2145669102041778,
                -4.150551578012804,
                3.51953904534754e-09,
            ),
            (
                0.04283185697010605,
                -4.575316215934372,
                1.597977195830182e-11,
            ),
        ];
        for (x, y, pdf) in data.iter().cloned() {
            assert_eq!(StandardNormal.density(&(x, y), &(0.0, 0.0), &h), pdf);
        }
    }
}
