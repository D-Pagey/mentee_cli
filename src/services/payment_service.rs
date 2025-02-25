use inquire::{CustomType, DateSelect};
use rusqlite::Connection;

use crate::{
    error::MenteeError,
    models::payment::PaymentWithMenteeName,
    repositories::{MenteeRepository, PaymentRepository},
    utils::validation::parse_date_from_db,
};

pub struct PaymentService<'a> {
    payment_repo: PaymentRepository<'a>,
    mentee_repo: MenteeRepository<'a>,
}

impl<'a> PaymentService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            payment_repo: PaymentRepository::new(conn),
            mentee_repo: MenteeRepository::new(conn),
        }
    }

    pub fn get_all_payments(
        &self,
        name: Option<String>,
    ) -> Result<Vec<PaymentWithMenteeName>, MenteeError> {
        let mentee_id = if let Some(name) = name {
            match self.mentee_repo.get_mentee_id(&name)? {
                Some(id) => Some(id),
                None => {
                    return Err(MenteeError::NotFound(format!(
                        "No mentee found with name '{}'",
                        name
                    )))
                }
            }
        } else {
            None
        };

        self.payment_repo
            .get_all_payments(mentee_id)
            .map_err(MenteeError::DatabaseError)
    }

    pub fn add_payment(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = self
            .mentee_repo
            .get_mentee_id(&name)?
            .ok_or_else(|| MenteeError::NotFound(format!("Mentee with name {}", name)))?;

        let date = DateSelect::new("Enter the date of the payment:")
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let amount: u32 = CustomType::new("Enter the payment amount:")
            .with_placeholder("e.g., 100")
            .prompt()
            .expect("Failed to read amount");

        match self.payment_repo.add_payment(mentee_id, date, amount) {
            Ok(..) => Ok(format!("Payment of {amount} added for {name}")),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }

    pub fn update_payment(&self, payment_id: u32) -> Result<String, MenteeError> {
        let payment = self
            .payment_repo
            .get_payment_by_id(payment_id)
            .map_err(|_| MenteeError::NotFound(format!("Payment with id {}", payment_id)))?;

        let parsed = parse_date_from_db(&payment.date).unwrap();

        let date = DateSelect::new("Enter the date of the payment:")
            .with_default(parsed)
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let amount: u32 = CustomType::new("How much?")
            .with_starting_input(&payment.amount.to_string())
            .prompt()
            .expect("Failed to read amount");

        match self.payment_repo.update_payment(&date, amount, payment_id) {
            Ok(_) => Ok(format!("Payment updated to {amount} on {date}")),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }

    pub fn delete_payment(&self, payment_id: u32) -> Result<String, MenteeError> {
        match self.payment_repo.delete_payment(payment_id) {
            Ok(0) => Err(MenteeError::NotFound(format!(
                "Payment with id {}",
                payment_id
            ))),
            Ok(_) => Ok(format!("Payment with id {} deleted", payment_id)),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }
}
