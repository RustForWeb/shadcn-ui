use yew::prelude::*;

use crate::new_york::components::ui::table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};

struct Invoice {
    invoice: &'static str,
    payment_status: &'static str,
    total_amount: &'static str,
    payment_method: &'static str,
}

fn invoices() -> Vec<Invoice> {
    vec![
        Invoice {
            invoice: "INV001",
            payment_status: "Paid",
            total_amount: "$250.00",
            payment_method: "Credit Card",
        },
        Invoice {
            invoice: "INV002",
            payment_status: "Pending",
            total_amount: "$150.00",
            payment_method: "PayPal",
        },
        Invoice {
            invoice: "INV003",
            payment_status: "Unpaid",
            total_amount: "$350.00",
            payment_method: "Bank Transfer",
        },
        Invoice {
            invoice: "INV004",
            payment_status: "Paid",
            total_amount: "$450.00",
            payment_method: "Credit Card",
        },
        Invoice {
            invoice: "INV005",
            payment_status: "Paid",
            total_amount: "$550.00",
            payment_method: "PayPal",
        },
        Invoice {
            invoice: "INV006",
            payment_status: "Pending",
            total_amount: "$200.00",
            payment_method: "Bank Transfer",
        },
        Invoice {
            invoice: "INV007",
            payment_status: "Unpaid",
            total_amount: "$300.00",
            payment_method: "Credit Card",
        },
    ]
}

#[function_component]
pub fn TableDemo() -> Html {
    html! {
        <Table>
            <TableCaption>{"A list of your recent invoices."}</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead class="w-[100px]">{"Invoice"}</TableHead>
                    <TableHead>{"Status"}</TableHead>
                    <TableHead>{"Method"}</TableHead>
                    <TableHead class="text-right">{"Amount"}</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {invoices().into_iter().map(|invoice| html! {
                    <TableRow key={invoice.invoice}>
                        <TableCell class="font-medium">{invoice.invoice}</TableCell>
                        <TableCell>{invoice.payment_status}</TableCell>
                        <TableCell>{invoice.payment_method}</TableCell>
                        <TableCell class="text-right">{invoice.total_amount}</TableCell>
                    </TableRow>
                }).collect::<Html>()}
            </TableBody>
            <TableFooter>
                <TableRow>
                    <TableCell colspan="3">{"Total"}</TableCell>
                    <TableCell class="text-right">{"$2,500.00"}</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
    }
}
