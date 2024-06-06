use chrono::{NaiveDate};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Reserva {
    nome_hotel: String,
    numero_quarto: u32,
    data_check_in: NaiveDate,
    data_check_out: NaiveDate,
}

struct SistemaReservas {
    reservas: HashMap<u64, Vec<Reserva>>,
}

impl SistemaReservas {
    fn new() -> Self {
        SistemaReservas {
            reservas: HashMap::new(),
        }
    }

    fn gerar_hash(&self, codigo_reserva: &str) -> u64 {
        let mut hash = 0u64;
        for byte in codigo_reserva.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    fn inserir_reserva(&mut self, codigo_reserva: &str, reserva: Reserva) {
        let hash = self.gerar_hash(codigo_reserva);
        self.reservas.entry(hash).or_insert_with(Vec::new).push(reserva);
    }

    fn recuperar_reserva(&self, codigo_reserva: &str) -> Option<&Vec<Reserva>> {
        let hash = self.gerar_hash(codigo_reserva);
        self.reservas.get(&hash)
    }

    fn remover_reserva(&mut self, codigo_reserva: &str) -> bool {
        let hash = self.gerar_hash(codigo_reserva);
        if let Some(reservas) = self.reservas.get_mut(&hash) {
            if !reservas.is_empty() {
                reservas.remove(0);
                if reservas.is_empty() {
                    self.reservas.remove(&hash);
                }
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut sistema = SistemaReservas::new();

    let reserva1 = Reserva {
        nome_hotel: "Hotel A".to_string(),
        numero_quarto: 101,
        data_check_in: NaiveDate::from_ymd(2024, 7, 1),
        data_check_out: NaiveDate::from_ymd(2024, 7, 5),
    };

    let reserva2 = Reserva {
        nome_hotel: "Hotel B".to_string(),
        numero_quarto: 202,
        data_check_in: NaiveDate::from_ymd(2024, 7, 3),
        data_check_out: NaiveDate::from_ymd(2024, 7, 8),
    };

    sistema.inserir_reserva("ABC123", reserva1.clone());
    sistema.inserir_reserva("DEF456", reserva2.clone());

    println!("Detalhes da reserva ABC123: {:?}", sistema.recuperar_reserva("ABC123"));
    println!("Detalhes da reserva DEF456: {:?}", sistema.recuperar_reserva("DEF456"));

    sistema.remover_reserva("ABC123");
    println!("Detalhes da reserva ABC123 após remoção: {:?}", sistema.recuperar_reserva("ABC123"));
}
