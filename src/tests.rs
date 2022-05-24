#[allow(dead_code)]
#[allow(unused)]
use crate::vector;
use crate::prop;
use crate::job;

#[test]
fn test_zero() {
    let a = vector::Vector::<3>::new();
    assert_eq!(&[0_f32; 3], a.components());
}

#[test]
fn test_from() {
    let a = vector::Vector::<3>::from([1., 3.3, 42.42]);
    assert_eq!(&[1., 3.3, 42.42], a.components());
}

#[test]
fn test_plus() {
    let mut a = vector::Vector::<3>::new();
    let b = vector::Vector::<3>::from([1., 2., 3.]);
    a.plus(&b);
    assert_eq!(&[1., 2., 3.], a.components());
    a.plus(&vector::Vector::<3>::from([-1., -2., -3.]));
    assert_eq!(&[0_f32; 3], a.components());
}

#[test]
fn test_multiply_by() {
    let mut a = vector::Vector::<3>::from([3.3, 2.02, 42.]);
    a.multiply_by(-1_f32);
    assert_eq!(&[-3.3, -2.02, -42.], a.components());
    a.multiply_by(1_f32);
    assert_eq!(&[-3.3, -2.02, -42.], a.components());
    a.multiply_by(-2_f32);
    assert_eq!(&[6.6, 4.04, 84.], a.components());
}

#[test]
fn test_dot() {
    let a = vector::Vector::<3>::from([1., 2., 3.]);
    let b = vector::Vector::<3>::from([2_f32; 3]);
    assert_eq!(12_f32, a.dot(&b));
    assert_eq!(14_f32, a.dot(&a));
    assert_eq!(12_f32, b.dot(&b));
}

#[test]
fn test_empty_mol() {
    let mut test = job::Job::<3>::default();
    test.run();
}