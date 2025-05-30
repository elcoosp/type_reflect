pub const SCOPE: &'static str = "test_case_inflection";

mod common;
mod tagged {

    use super::common::*;
    use anyhow::Result;

    use super::SCOPE;
    use serde::{Deserialize, Serialize};
    use type_reflect::*;

    #[derive(Reflect, Serialize, Deserialize)]
    pub struct Rectangle {
        width: f32,
        height: f32,
    }

    #[derive(Reflect, Serialize, Deserialize)]
    #[serde(tag = "_case", content = "data")]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Shape {
        Circle { radius: f32 },
        Square { side: f32 },
        Rectangle(Rectangle),
        ScaledRectangle(Rectangle, u32),
        Null,
    }

    #[test]
    fn test_screaming_snake() -> Result<()> {
        let output = init_path(SCOPE, "test_screaming_snake");

        export_types!(
            types: [ Shape, Rectangle ],
            destinations: [(
                output.ts_path(),
                emitters: [
                    TypeScript(),
                    TSValidation(),
                    TSFormat(
                        tab_size: 2,
                        line_width: 80,
                    ),
                ],
            )]
        )?;

        output.write_jest(
        "Shape, Rectangle, ShapeCase, ShapeCaseKey",
        ts_string! {
            describe("ADT Validation", ()=>{
                it("Validates a Null variant: ShapeCaseKey.Null", ()=>{
                    expect(() => {
                        Shape.validate({_case: ShapeCaseKey.Null})
                    }).not.toThrow();
                    expect(() => {
                        Shape.validate({_case: "NULL"})
                    }).not.toThrow();
                });
                it("Validates a Circle variant: {_case: ShapeCaseKey.Circle, data: { radius: 1.7} }", ()=>{
                    expect(() => {
                        Shape.validate({
                            _case: ShapeCaseKey.Circle,
                            data: {
                                radius: 1.7
                            }
                        })
                    }).not.toThrow();
                    expect(() => {
                        Shape.validate({
                            _case: "CIRCLE",
                            data: {
                                radius: 1.7
                            }
                        })
                    }).not.toThrow();
                });
                it("Validates a Rectangle variant: {_case: ShapeCaseKey.Rectangle, data: { width: 1, height: 2} }", ()=>{
                    expect(() => {
                        Shape.validate({
                            _case: ShapeCaseKey.Rectangle,
                            data: {
                                width: 1,
                                height: 2
                            }
                        })
                    }).not.toThrow();
                    expect(() => {
                        Shape.validate({
                            _case: "RECTANGLE",
                            data: {
                                width: 1,
                                height: 2
                            }
                        })
                    }).not.toThrow();
                });
                it("Validates a ScaledRectangle variant: {_case: ShapeCaseKey.ScaledRectangle, data: [{ width: 1, height: 2}, 0.5] }", ()=>{
                    expect(() => {
                        Shape.validate({
                            _case: ShapeCaseKey.ScaledRectangle,
                            data: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).not.toThrow();
                    expect(() => {
                        Shape.validate({
                            _case: "SCALED_RECTANGLE",
                            data: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).not.toThrow();
                });
                it("Doesn't Validate an incorrect ScaledRectangle variant: {_case: ShapeCaseKey.Circle, data: [{ width: 1, height: 2}, 0.5] }", ()=>{
                    expect(() => {
                        Shape.validate({
                            _case: ShapeCaseKey.Circle,
                            data: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).toThrow();
                    expect(() => {
                        Shape.validate({
                            _case: "CIRCLE",
                            data: [
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

        output.run_ts()
    }
}

mod untagged {

    use super::common::*;
    use anyhow::Result;

    use super::SCOPE;
    use serde::{Deserialize, Serialize};
    use type_reflect::*;

    #[derive(Serialize, Deserialize, Reflect)]
    pub struct Rectangle {
        width: f32,
        height: f32,
    }

    #[derive(Serialize, Deserialize, Reflect)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Shape {
        Circle { radius: f32 },
        Square { side: f32 },
        Rectangle(Rectangle),
        Scale(f32),
        ScaledRectangle(Rectangle, f32),
        Null,
    }

    #[test]
    fn test_untagged_screaming_snake() -> Result<()> {
        let output = init_path(SCOPE, "test_untagged_screaming_snake");

        // let value = Shape::Circle { radius: 5.0 };
        // let json = serde_json::to_string_pretty(&value)?;
        // println!("{json}");

        // let value = Shape::Rectangle(Rectangle {
        //     width: 5.0,
        //     height: 5.0,
        // });
        // let json = serde_json::to_string_pretty(&value)?;
        // println!("{json}");

        // let value = Shape::ScaledRectangle(
        //     Rectangle {
        //         width: 5.0,
        //         height: 5.0,
        //     },
        //     2.0,
        // );
        // let json = serde_json::to_string_pretty(&value)?;
        // println!("{json}");

        // let value = Shape::Scale(2.0);
        // let json = serde_json::to_string_pretty(&value)?;
        // println!("{json}");

        // let value = Shape::Null;
        // let json = serde_json::to_string_pretty(&value)?;
        // println!("{json}");

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
                            Shape.validate("NULL")
                        }).not.toThrow();
                    });
                    it("Validates a Circle variant:", ()=>{
                        expect(() => {
                            Shape.validate({
                                CIRCLE: {
                                    radius: 1.7
                                }
                            })
                        }).not.toThrow();
                    });
                    it("Validates a Rectangle variant:", ()=>{
                        expect(() => {
                            Shape.validate({
                                RECTANGLE: {
                                    width: 1,
                                    height: 2
                                }
                            })
                        }).not.toThrow();
                    });
                    it("Validates a ScaledRectangle variant:", ()=>{
                        expect(() => {
                            Shape.validate({

                                SCALED_RECTANGLE: [
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
                                CIRCLE: [
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
}
