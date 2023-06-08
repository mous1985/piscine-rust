pub mod areas_volumes;
pub use crate::areas_volumes::*;
pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    let rectangle_surface = x * y;
    let object_area = match objects {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Triangle => triangle_area(a, b) as usize,
    };
    object_area * times <= rectangle_surface
}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let box_volume = x * y * z;
    let object_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a),
        GeometricalVolumes::Sphere => sphere_volume(a) as usize,
        GeometricalVolumes::Cone => cone_volume(a, b) as usize,
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b) as usize,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c),
    };
    object_volume * times <= box_volume
}