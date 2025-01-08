use leptos::prelude::*;

use crate::new_york::components::ui::table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};

#[derive(Clone, Copy)]
struct PaymentDetails{
    payment_status: &'static str,
    total_amount: &'static str,
    payment_method: &'static str,
}

#[derive(Clone, Copy)]
struct Invoice {
    invoice: &'static str,
    details: RwSignal<PaymentDetails>,
}

fn invoices() -> Vec<Invoice> {
    vec![
        Invoice {
            invoice: "INV001",
            details: RwSignal::new(PaymentDetails {
                payment_status: "Paid",
                total_amount: "$250.00",
                payment_method: "Credit Card",    
            })
            
        },
        Invoice {
            invoice: "INV002",
            details: RwSignal::new(PaymentDetails {
                payment_status: "Pending",
                total_amount: "$150.00",
                payment_method: "PayPal",    
            })
            
        },
        Invoice {
            invoice: "INV003",
            details: RwSignal::new(PaymentDetails {    
                payment_status: "Unpaid",
                total_amount: "$350.00",
                payment_method: "Bank Transfer",
            })
        },
        Invoice {
            invoice: "INV004",
            details: RwSignal::new(PaymentDetails {
                payment_status: "Paid",
                total_amount: "$450.00",
                payment_method: "Credit Card",    
            })
            
        },
        Invoice {
            invoice: "INV005",
            details: RwSignal::new(PaymentDetails {
                payment_status: "Paid",
                total_amount: "$550.00",
                payment_method: "PayPal",    
            })
            
        },
        Invoice {
            invoice: "INV006",
            details: RwSignal::new(PaymentDetails {
                payment_status: "Pending",
                total_amount: "$200.00",
                payment_method: "Bank Transfer",    
            })
            
        },
        Invoice {
            invoice: "INV007",
            details: RwSignal::new(PaymentDetails {
                payment_status: "Unpaid",
                total_amount: "$300.00",
                payment_method: "Credit Card",    
            })
        },
    ]
}

#[component]
pub fn TableDemo() -> impl IntoView {
    view! {
        <Table>
            <TableCaption>"A list of your recent invoices."</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead class="w-[100px]">"Invoice"</TableHead>
                    <TableHead>"Status"</TableHead>
                    <TableHead>"Method"</TableHead>
                    <TableHead class="text-right">"Amount"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <For 
                    each=move || invoices()
                    key=|row| row.invoice
                    children=move |row| {
                        let details = row.details.get();
                        view! {
                            <TableRow>
                                <TableCell class="font-medium">{row.invoice}</TableCell>
                                <TableCell>{details.payment_status}</TableCell>
                                <TableCell>{details.payment_method}</TableCell>
                                <TableCell class="text-right">{details.total_amount}</TableCell>
                            </TableRow>
                        }
                    }
                />
            </TableBody>

            <TableFooter>
                <TableRow>
                    <TableCell colspan="3">"Total"</TableCell>
                    <TableCell class="text-right">"$2,500.00"</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
    }
}
