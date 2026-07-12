trait Orbitar {
    fn describir_orbita(&self);
    fn avanzar_orbita(&mut self, dias: &u32);
}

struct Planeta {
    nombre: String,
    periodo_orbital: f64,
    dias_transcurridos: u32,
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
    let tiempo_simulacion = 90;
    println!("-------- Iniciando la Simulacion -----------");
    simular_dia_en_el_cosmos(&mut tierra, &tiempo_simulacion);
    let mut factor_velocidad = 2;
    println!("\n¡Hipersalto activado! Multiplicador de tiempo actual: {}x", factor_velocidad);
    let nuevo_tiempo = tiempo_simulacion*factor_velocidad;
    simular_dia_en_el_cosmos(&mut tierra, &nuevo_tiempo);
}