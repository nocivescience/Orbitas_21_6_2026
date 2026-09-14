trait Orbitar {
    fn describir_orbita(&self);
    fn avanzar_orbita(&mut self, dias: &u32);
}

struct Planeta {
    nombre: String,
    periodo_orbital: f64,
    dias_transcurridos: u32,
}

struct Estrella {
    nombre: String,
    tipo: String,
    edad: u32,
}

struct Cometa {
    nombre: String,
    periodo_orbital: f64,
    dias_transcurridos: u32,
}

impl Orbitar for Estrella {
    fn describir_orbita(&self) {
        println!("La estrella {} no tiene una órbita definida, pero es de tipo {} y tiene {} años.", self.nombre, self.tipo, self.edad);
    }
    fn avanzar_orbita(&mut self, _dias: &u32) {
        println!("Las estrellas no avanzan en órbita como los planetas.");
    }
}

impl Orbitar for Planeta {
    fn describir_orbita(&self) {
        let progreso = (self.dias_transcurridos as f64 /self.periodo_orbital as f64) * 100.0;
        println!("El planeta {} ha completado el {:.2}% de un year", self.nombre, progreso);
    }
    fn avanzar_orbita(&mut self, dias: &u32) {
        self.dias_transcurridos += *dias;
    }
}

impl Orbitar for Cometa {
    fn describir_orbita(&self){
        let progresos= (self.periodo_orbital as f64+ self.dias_transcurridos as f64);
        println!("El cometa: {} ha completado {:.2} de su viaje", self.nombre, progresos )
    }
    fn avanzar_orbita(&mut self, dias: &u32) {
        self.dias_transcurridos+= *dias;
    }
}

fn simular_dia_en_el_cosmos(cuerpo: &mut impl Orbitar,dias_a_pasar: &u32) {
    cuerpo.avanzar_orbita(dias_a_pasar);
    cuerpo.describir_orbita();
}

fn main() {
    let mut tierra = Planeta{
        nombre: String::from("Tierra"),
        periodo_orbital: 365.25,
        dias_transcurridos: 0,
    };
    let mut estrella = Estrella{
        nombre: String::from("Sol"),
        tipo: String::from("G2V"),
        edad: 4600000,
    };
    let mut cometa= Cometa{
        nombre: String::from("Halley"),
        periodo_orbital: 7.0,
        dias_transcurridos: 70,
    };
    let tiempo_simulacion = 90;
    println!("-------- Iniciando la Simulacion -----------");
    simular_dia_en_el_cosmos(&mut tierra, &tiempo_simulacion);
    let factor_velocidad = 2;
    println!("\n¡Hipersalto activado! Multiplicador de tiempo actual: {}x", factor_velocidad);
    let nuevo_tiempo = tiempo_simulacion*factor_velocidad;
    simular_dia_en_el_cosmos(&mut tierra, &nuevo_tiempo);
    let factor_luminario = 5;
    let factor_transito= 12 as u32;
    println!("\nLuminocidad ultra roja: {} lum", factor_luminario);
    simular_dia_en_el_cosmos(&mut estrella, &factor_luminario);
    simular_dia_en_el_cosmos(&mut cometa, &factor_transito);
}