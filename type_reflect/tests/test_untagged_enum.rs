mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Serialize, Deserialize, Reflect)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

#[derive(Serialize, Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub enum Shape {
    Circle { radius: f32 },
    Square { side: f32 },
    Rectangle(Rectangle),
    Scale(f32),
    ScaledRectangle(Rectangle, f32),
    Null,
}

pub const SCOPE: &'static str = "test_untagged_enum";

#[test]
fn test_validation() -> Result<()> {
    let output = init_path(SCOPE, "test_validation");

    let value = Shape::Circle { radius: 5.0 };
    let json = serde_json::to_string_pretty(&value)?;
    println!("{json}");

    let value = Shape::Rectangle(Rectangle {
        width: 5.0,
        height: 5.0,
    });
    let json = serde_json::to_string_pretty(&value)?;
    println!("{json}");

    let value = Shape::ScaledRectangle(
        Rectangle {
            width: 5.0,
            height: 5.0,
        },
        2.0,
    );
    let json = serde_json::to_string_pretty(&value)?;
    println!("{json}");

    let value = Shape::Scale(2.0);
    let json = serde_json::to_string_pretty(&value)?;
    println!("{json}");

    let value = Shape::Null;
    let json = serde_json::to_string_pretty(&value)?;
    println!("{json}");

    export_types!(
        types: [Rectangle,  Shape],
        destinations: [(
            output.ts_path(),
            emitters: [
                TypeScript(),
                TSValidation(),
                TSFormat(
                    tab_size: 2,
                    line_width: 60,
                ),
            ],
        )]
    )?;

    output.write_jest(
        "Shape, Rectangle",
        ts_string! {
            describe("ADT Validation", ()=>{
                it("Validates a Null variant:", ()=>{
                    expect(() => {
                        Shape.validate("null")
                    }).not.toThrow();
                });
                it("Validates a Circle variant:", ()=>{
                    expect(() => {
                        Shape.validate({
                            circle: {
                                radius: 1.7
                            }
                        })
                    }).not.toThrow();
                });
                it("Validates a Rectangle variant:", ()=>{
                    expect(() => {
                        Shape.validate({
                            rectangle: {
                                width: 1,
                                height: 2
                            }
                        })
                    }).not.toThrow();
                });
                it("Validates a ScaledRectangle variant:", ()=>{
                    expect(() => {
                        Shape.validate({

                            scaledRectangle: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).not.toThrow();
                });
                it("Doesn't Validate an incorrect ScaledRectangle variant:", ()=>{
                    expect(() => {
                        Shape.validate({
                            circle: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).toThrow();
                });

            });
        }
        .as_str(),
    )?;

    output.run_ts().unwrap();

    Ok(())
}
