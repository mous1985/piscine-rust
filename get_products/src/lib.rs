pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 1 {
        return vec![];
    }
    if n == 0 {
        return vec![];
    }

    let mut products = vec![1; n];

    let mut prefix_product = 1;
    for i in 0..n {
        products[i] *= prefix_product;
        prefix_product *= arr[i];
    }

    let mut suffix_product = 1;
    for i in (0..n).rev() {
        products[i] *= suffix_product;
        suffix_product *= arr[i];
    }

    products
}