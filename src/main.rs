const G_CONSTANT: f32 = 6.67430e-11; // Constante Gravitacional
const TIME_STEP: f32 = 0.1;

// Criando um modulo para poder utilizar vetores
mod vector{
	#[derive(Clone)] // Implementa o trait Clone para a estrutura Vector
	// Estrutura do Vetor
	pub struct Vector{
		pub x: f32,
		pub y: f32,
		pub z: f32,
	}

	impl Vector{
		
		// Cria um novo vetor
		pub fn new(x: f32, y: f32, z:f32) -> Vector{
			Vector{x,y,z}
		}
	
		// Calcula distância entre dois vetores
		pub fn distance(&self, other: &Vector) -> f32{
			((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
		}

		// Adiciona um vetor a outro
		pub fn add(&mut self, other: Vector) -> &Vector{
			self.x += other.x;
			self.y += other.y;
			self.z += other.z;
			self
		}
		
		// Subtrai um vetor de outro
		pub fn subtract(&mut self, other: Vector) -> &Vector{
			self.x -= other.x;
			self.y -= other.y;
			self.z -= other.z;
			self
		}
		
		// Multiplica o vetor por um escalar
		pub fn scale(&mut self, scalar: f32) -> &Vector{
			self.x *= scalar;
			self.y *= scalar;
			self.z *= scalar;
			self
		}

		// Seta os valores neste vetor
		pub fn set(&mut self, x: f32, y: f32, z: f32) -> &Vector{
			self.x = x;
			self.y = y;
			self.z = z;
			self
		}

	}

}

// Estrutura de um planeta
struct Body {

	position: vector::Vector, // Vetor posição
	velocity: vector::Vector, // Vetir velocidade

	mass: f32, // Massa do planeta
}

// Funções para a estrutura do Planeta
impl Body{
	// Cria o corpo
	fn new(x: f32, y: f32, z: f32, mass: f32) -> Body{
		Body{
			position: vector::Vector::new(x, y, z),
			velocity: vector::Vector::new(0.0,0.0,0.0),
			mass,
		}
	}
	
	// Calcula a aceleração gravitacional exercida por outro corpo
	fn gravitational_acceleration(&self, other: &Body) -> vector::Vector {
		let mut r_vector = vector::Vector::new(
			other.position.x - self.position.x,
			other.position.y - self.position.y,
			other.position.z - self.position.z,
		);
		let distance = self.position.distance(&other.position);
		let r_cubed = distance.powi(3);
		let scalar = G_CONSTANT * other.mass / r_cubed;
		r_vector.scale(scalar);
		r_vector
	}
	
	// Atualiza a posição e a velocidade do corpo
	fn update(&mut self, acceleration: &mut vector::Vector, time_step: f32) {
		// Atualiza a velocidade
		self.velocity.add(acceleration.scale(time_step).clone());

		// Atualiza a posição
		self.position.add(self.velocity.scale(time_step).clone());
	}
	
}

fn main() {
	let mut planet1 = Body::new(0.0, 0.0, 0.0, 5.1e24); // Planeta com massa 5.1e24 kg
	let mut planet2 = Body::new(10.0, 10.0, 10.0, 15.1e24); // Planeta com massa 15.1e24 kg

	let mut t: f32 = 0.0;

	// Simulação por 1000 passos de tempo
	for _ in 0..864000 {
		t += TIME_STEP;
		let mut acceleration1 = planet1.gravitational_acceleration(&planet2);
		let mut acceleration2 = planet2.gravitational_acceleration(&planet1);

		planet1.update(&mut acceleration1, TIME_STEP);
		planet2.update(&mut acceleration2, TIME_STEP);

        println!(
            "Planeta 1 - Posição: ({:.2}, {:.2}, {:.2}) Velocidade: ({:.2}, {:.2}, {:.2})",
            planet1.position.x, planet1.position.y, planet1.position.z,
            planet1.velocity.x, planet1.velocity.y, planet1.velocity.z
        );

        println!(
            "Planeta 2 - Posição: ({:.2}, {:.2}, {:.2}) Velocidade: ({:.2}, {:.2}, {:.2})",
            planet2.position.x, planet2.position.y, planet2.position.z,
            planet2.velocity.x, planet2.velocity.y, planet2.velocity.z
        );
        
        println!("Tempo {}",t);
    }
}
