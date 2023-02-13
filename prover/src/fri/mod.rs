use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use lambdaworks_math::polynomial::Polynomial;

const ORDER: u64 = 293;
pub type F = U64PrimeField<ORDER>;
pub type FE = FieldElement<F>;

fn fold_polynomial(
    poly: &Polynomial<FieldElement<F>>,
    beta: &FieldElement<F>,
) -> Polynomial<FieldElement<F>> {
    let coef = poly.coefficients();
    let even_coef: Vec<FieldElement<F>> = coef
        .iter()
        .enumerate()
        .filter(|(pos, _)| pos % 2 == 0)
        .map(|(_pos, v)| *v)
        .collect();

    // odd coeficients of poly are multiplied by beta
    let odd_coef_mul_beta: Vec<FieldElement<F>> = coef
        .iter()
        .enumerate()
        .filter(|(pos, _)| pos % 2 == 1)
        .map(|(_pos, v)| (*v) * beta)
        .collect();

    let (even_poly, odd_poly) = Polynomial::pad_with_zero_coefficients(
        &Polynomial::new(&even_coef),
        &Polynomial::new(&odd_coef_mul_beta),
    );
    even_poly + odd_poly
}

fn next_domain(input: &[FE]) -> Vec<FE> {
    let length = input.len() / 2;
    let mut ret = Vec::with_capacity(length);
    for v in input.iter().take(length) {
        ret.push(v * v)
    }
    ret
}

fn next_layer(poly: &Polynomial<FieldElement<F>>, domain: &[FE]) -> Vec<FE> {
    let length = domain.len() / 2;
    let mut ret = Vec::with_capacity(length);
    for v in domain.iter() {
        ret.push(poly.evaluate(v));
    }
    ret
}

/// Returns:
/// * new polynomoial folded with FRI protocol
/// * new domain
/// * evaluations of the polynomial
pub fn next_fri_layer(
    poly: &Polynomial<FieldElement<F>>,
    domain: &[FE],
    beta: &FieldElement<F>,
) -> (Polynomial<FieldElement<F>>, Vec<FE>, Vec<FE>) {
    let ret_poly = fold_polynomial(poly, beta);
    let ret_next_domain = next_domain(domain);
    let ret_evaluation = next_layer(&ret_poly, &ret_next_domain);
    (ret_poly, ret_next_domain, ret_evaluation)
}

#[cfg(test)]
mod tests {
    use super::{fold_polynomial, next_domain, next_fri_layer, next_layer, FieldElement, F, FE};
    use lambdaworks_math::polynomial::Polynomial;

    #[test]
    fn test_fold() {
        let p0 = Polynomial::new(&[
            FE::new(3),
            FE::new(1),
            FE::new(2),
            FE::new(7),
            FE::new(3),
            FE::new(5),
        ]);
        let beta = FieldElement::<F>::new(4);
        let p1 = fold_polynomial(&p0, &beta);
        assert_eq!(
            p1,
            Polynomial::new(&[FE::new(7), FE::new(30), FE::new(23),])
        );

        let gamma = FieldElement::<F>::new(3);
        let p2 = fold_polynomial(&p1, &gamma);
        assert_eq!(p2, Polynomial::new(&[FE::new(97), FE::new(23),]));

        let delta = FieldElement::<F>::new(2);
        let p3 = fold_polynomial(&p2, &delta);
        assert_eq!(p3, Polynomial::new(&[FE::new(143)]));
        assert_eq!(p3.degree(), 0);
    }

    #[test]
    fn test_next_domain() {
        let input = [
            FE::new(5),
            FE::new(7),
            FE::new(13),
            FE::new(20),
            FE::new(1),
            FE::new(1),
            FE::new(1),
            FE::new(1),
        ];
        let ret_next_domain = next_domain(&input);
        assert_eq!(
            ret_next_domain,
            &[FE::new(25), FE::new(49), FE::new(169), FE::new(107),]
        );

        let ret_next_domain_2 = next_domain(&ret_next_domain);
        assert_eq!(ret_next_domain_2, &[FE::new(39), FE::new(57)]);

        let ret_next_domain_3 = next_domain(&ret_next_domain_2);
        assert_eq!(ret_next_domain_3, &[FE::new(56)]);
    }

    #[test]
    fn test_next_layer() {
        let p0 = Polynomial::new(&[
            FE::new(3),
            FE::new(1),
            FE::new(2),
            FE::new(7),
            FE::new(3),
            FE::new(5),
        ]);

        let domain = [FE::new(5), FE::new(9)];
        let eval = next_layer(&p0, &domain);

        assert_eq!(eval, [FE::new(267), FE::new(249)]);
    }

    #[test]
    fn text_next_fri_layer() {
        let p0 = Polynomial::new(&[
            FE::new(3),
            FE::new(1),
            FE::new(2),
            FE::new(7),
            FE::new(3),
            FE::new(5),
        ]);
        let beta = FieldElement::<F>::new(4);
        let input_domain = [
            FE::new(5),
            FE::new(7),
            FE::new(13),
            FE::new(20),
            FE::new(1),
            FE::new(1),
            FE::new(1),
            FE::new(1),
        ];

        let (p1, ret_next_domain, ret_evaluation) = next_fri_layer(&p0, &input_domain, &beta);

        assert_eq!(
            p1,
            Polynomial::new(&[FE::new(7), FE::new(30), FE::new(23),])
        );
        assert_eq!(
            ret_next_domain,
            &[FE::new(25), FE::new(49), FE::new(169), FE::new(107),]
        );
        assert_eq!(
            ret_evaluation,
            &[FE::new(189), FE::new(151), FE::new(93), FE::new(207),]
        );
    }
}
