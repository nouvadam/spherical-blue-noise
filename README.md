# Spherical blue noise generator

Library for generating points with blue noise distribution on a unit sphere.

The underlying idea for this algorithm is:
* First generate points on a sphere with random (white) distribution.
* Then treat each point as physically, charged particle and apply to each one repulsive force from other particles.
* With time, particles converges to the equilibrium that resembles blue noise.

Hence, the time complexity of this algorithm is O(N^2), where N is the number of points. (could be made faster by using octree, like in barnes-hut algorithm)

# Visualization
![plot](https://user-images.githubusercontent.com/66559370/105104550-c3c3ce00-5ab2-11eb-987a-65f34b1098b7.gif)

Code for this animation is available in the `examples` folder.

# Example

````rust
use spherical_blue_noise::*;

let blue_noise_vec: Vec<(f32, f32, f32)> = BlueNoiseSphere::new(16, &mut rand::thread_rng()).into_iter().collect();
println!("{:?}", blue_noise_vec);

 ````
# Reference
The basic idea is based on the paper:

Wong, Kin-Ming and Wong, Tien-Tsin. "Spherical Blue Noise", Pacific Graphics Short Papers, 2018,
 [link](https://diglib.eg.org/handle/10.2312/pg20181267)
