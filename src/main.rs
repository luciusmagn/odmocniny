/// ## Odmociny
/// Jednoduchá kalkulačka odmocnin.
///
/// Není úplně přesná, část přesnosti se ztrácí kvůli reprezentaci desetinných čísel s plavoucí čárkou v paměti.
/// Do stdout vypisuje pouze čísla, všechny ostatní zprávy jsou v stderr tak, aby mohl být výstup programu
/// využit jinde
///
/// ### Kompilace:
/// ```bash
/// cargo build --release
/// ```
/// Binární soubor bude target/release/odmociny
///
/// ### Spouštení přes Cargo
/// ```bash
/// cargo run --release
/// ```
/// moje prostředé: Arch Linux x86_64, GCC, Rust 1.34.0-nightly (c1d2d83ca 2019-03-01)
extern crate yansi;
extern crate promptly;

use yansi::Paint;
use promptly::{prompt, prompt_default};

/// jedna iterace Newtonovy metody, viz zadání
fn newton(x: f64, n: i32, b: f64) -> f64 {
	(((n - 1) as f64 * x) / n as f64)
	+
	(b / (n as f64 * x.powi(n - 1)))
}

fn main() {
	/// Záskání vstupu
	let cislo: f64 = prompt(format!("{}", Paint::yellow("Zadejte číslo")));
	let rad_c: u32 = prompt(format!("{}", Paint::yellow("Zadejte řád odmocniny")));
	let see: bool = prompt_default(format!("{}", Paint::yellow("Přejete si vidět jednotlivé kroky Newtonovy metody?")), false);

	eprintln!("{}", Paint::yellow(format!("Počítám {}. odmocninu z {}", rad_c, cislo)));

	// konverze typů
	let rad = rad_c as f64;
	let rad_c = rad_c as i32;

	//Xy, Xy-1
	let (mut x, mut x2) = (0f64, 1f64);

	// iterace
	while (x.abs() - x2.abs()).abs() > 0.0001 {
		if see {
			eprintln!("({}, {})", x, x2);
		}

		x = x2;
		x2 = newton(x, rad_c, cislo);
	}


	// Zaokrouhlení na patřičný počet desetinných míst
	let x2 = (x2 * 10000f64).round() / 10000f64;

	// Vypsání výsĺedků dle jednotlivých případů
	eprintln!("{}:", Paint::green("Výsledky"));
	match rad {
		_ if rad_c % 2 == 0 && cislo < 0f64 => println!("+-{0} +-{0}i", x2.abs()),
		_ if rad_c % 2 == 0 => println!("+-{}", x2.abs()),
		_ => println!("{}", x2),
	}
}
