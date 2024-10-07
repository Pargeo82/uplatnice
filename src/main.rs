use pdf417_hub3_rs::{generate, save, PaymentOrder, Receiver, Sender};
use rust_decimal::Decimal;
use std::error::Error;

use printpdf::image::{self};
use printpdf::{Image, Mm, PdfDocument};

use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
    // Define variables
    let currency = "EUR".to_string();
    let amount = Decimal::from(10000u64); // 100.00 EUR in cents

    // Sender Information
    let sender_name = "Platitelj Name".to_string();
    let sender_address = "Payer Street 123".to_string();
    let sender_city = "10000 Zagreb".to_string();

    // Receiver Information
    let receiver_name = "PARGEO DESIGN, vl. Mislav Markušić".to_string();
    let receiver_address = "Savska 39".to_string();
    let receiver_city = "10290 Zaprešić".to_string();
    let iban = "HR1210010051863000160".to_string();
    let model = "HR01".to_string();

    // Other Data
    let call_number = "12345678901".to_string();
    let payment_code = "COST".to_string();
    let payment_description = "Payment for services".to_string();

    // Create PaymentOrder
    let data = PaymentOrder::new(
        currency.clone(),
        amount,
        Sender::new(
            sender_name.clone(),
            sender_address.clone(),
            sender_city.clone(),
        ),
        Receiver::new(
            receiver_name.clone(),
            receiver_address.clone(),
            receiver_city.clone(),
            iban.clone(),
            model.clone(),
        ),
        call_number.clone(),
        payment_code.clone(),
        payment_description.clone(),
    );

    // Generate the barcode
    let barcode = generate(data)?; // Remove '&'

    // Save the barcode as an image (PNG)
    let barcode_filename = "barcode.png";
    save(barcode_filename, &barcode)?;

    // Create the PDF with the barcode
    create_pdf(
        barcode_filename,
        &receiver_name,
        &receiver_address,
        &receiver_city,
        &currency,
        amount,
        &iban,
        &model,
        &call_number,
        "uplatnica.pdf",
    )?;

    Ok(())
}

fn create_pdf(
    barcode_filename: &str,
    receiver_name: &str,
    receiver_address: &str,
    receiver_city: &str,
    currency: &str,
    amount: Decimal,
    iban: &str,
    model: &str,
    call_number: &str,
    output_pdf: &str,
) -> Result<(), Box<dyn Error>> {
    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new("Uplatnica", Mm(210.0), Mm(99.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load the barcode image
    let dyn_image = image::open(barcode_filename)?;
    let image = Image::from_dynamic_image(&dyn_image);

    // Add barcode to the document
    image.add_to_layer(
        current_layer.clone(),
        Some(Mm(10.0)), // x position
        Some(Mm(10.0)), // y position
        None,           // rotation
        Some(0.5),      // scale x (adjust as needed)
        Some(0.5),      // scale y (adjust as needed)
        None,           // DPI
    );

    // Load the font
    let font = doc.add_external_font(File::open("fonts/Roboto-Regular.ttf")?)?;

    // Add text to the invoice
    current_layer.use_text(
        format!("Primatelj: {}", receiver_name),
        12.0,
        Mm(70.0),
        Mm(80.0),
        &font,
    );
    current_layer.use_text(
        format!("Adresa: {}", receiver_address),
        12.0,
        Mm(70.0),
        Mm(75.0),
        &font,
    );
    current_layer.use_text(
        format!("Grad: {}", receiver_city),
        12.0,
        Mm(70.0),
        Mm(70.0),
        &font,
    );
    current_layer.use_text(
        format!("Iznos: {} {}", currency, amount / Decimal::new(100, 0)),
        12.0,
        Mm(70.0),
        Mm(65.0),
        &font,
    );
    current_layer.use_text(format!("IBAN: {}", iban), 12.0, Mm(70.0), Mm(60.0), &font);
    current_layer.use_text(format!("Model: {}", model), 12.0, Mm(70.0), Mm(55.0), &font);
    current_layer.use_text(
        format!("Poziv na broj: {}", call_number),
        12.0,
        Mm(70.0),
        Mm(50.0),
        &font,
    );

    // Save the PDF
    let output = File::create(output_pdf)?;
    let mut buf_output = BufWriter::new(output);
    doc.save(&mut buf_output)?;

    Ok(())
}
