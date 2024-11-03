<p align="center">
    <img src="./images/logo.svg" width="300" height="200" alt="Rust shadcn/ui Logo">
</p>

# Introduction

Rust shadcn/ui is a Rust port of [shadcn/ui](https://ui.shadcn.com/).

[shadcn/ui](https://ui.shadcn.com/) is a library of beautifully designed components that you can copy and paste into your apps.

## Frameworks

Rust shadcn/ui is available for the following frameworks:

-   [Leptos](https://leptos.dev/)
-   [Yew](https://yew.rs/)

The following frameworks are under consideration:

-   [Dioxus](https://dioxuslabs.com/)

The tables below show the support for the various frameworks.

-   ✅ = Supported
-   🟦 = Early Support
-   🚧 = Work In Progress
-   ❌ = Unsupported

### Primitives Support

| Name            | Dioxus                                                      | Leptos                                                        | Yew                                                           |
| --------------- | ----------------------------------------------------------- | ------------------------------------------------------------- | ------------------------------------------------------------- |
| Accordion       | ❌ [#12](https://github.com/RustForWeb/shadcn-ui/issues/12) | ❌ [#61](https://github.com/RustForWeb/shadcn-ui/issues/61)   | ❌ [#110](https://github.com/RustForWeb/shadcn-ui/issues/110) |
| Alert           | ❌ [#13](https://github.com/RustForWeb/shadcn-ui/issues/13) | ❌ [#62](https://github.com/RustForWeb/shadcn-ui/issues/62)   | 🟦 [#111](https://github.com/RustForWeb/shadcn-ui/issues/111) |
| Alert Dialog    | ❌ [#14](https://github.com/RustForWeb/shadcn-ui/issues/14) | ❌ [#63](https://github.com/RustForWeb/shadcn-ui/issues/63)   | ❌ [#112](https://github.com/RustForWeb/shadcn-ui/issues/112) |
| Aspect Ratio    | ❌ [#15](https://github.com/RustForWeb/shadcn-ui/issues/15) | ❌ [#64](https://github.com/RustForWeb/shadcn-ui/issues/64)   | ❌ [#113](https://github.com/RustForWeb/shadcn-ui/issues/113) |
| Avatar          | ❌ [#16](https://github.com/RustForWeb/shadcn-ui/issues/16) | ❌ [#65](https://github.com/RustForWeb/shadcn-ui/issues/65)   | ❌ [#114](https://github.com/RustForWeb/shadcn-ui/issues/114) |
| Badge           | ❌ [#17](https://github.com/RustForWeb/shadcn-ui/issues/17) | ❌ [#66](https://github.com/RustForWeb/shadcn-ui/issues/66)   | ❌ [#115](https://github.com/RustForWeb/shadcn-ui/issues/115) |
| Breadcrumb      | ❌ [#18](https://github.com/RustForWeb/shadcn-ui/issues/18) | ❌ [#67](https://github.com/RustForWeb/shadcn-ui/issues/67)   | ❌ [#116](https://github.com/RustForWeb/shadcn-ui/issues/116) |
| Button          | ❌ [#19](https://github.com/RustForWeb/shadcn-ui/issues/19) | ❌ [#68](https://github.com/RustForWeb/shadcn-ui/issues/68)   | 🟦 [#117](https://github.com/RustForWeb/shadcn-ui/issues/117) |
| Calendar        | ❌ [#20](https://github.com/RustForWeb/shadcn-ui/issues/20) | ❌ [#69](https://github.com/RustForWeb/shadcn-ui/issues/69)   | ❌ [#118](https://github.com/RustForWeb/shadcn-ui/issues/118) |
| Card            | ❌ [#21](https://github.com/RustForWeb/shadcn-ui/issues/21) | ❌ [#70](https://github.com/RustForWeb/shadcn-ui/issues/70)   | ❌ [#119](https://github.com/RustForWeb/shadcn-ui/issues/119) |
| Carousel        | ❌ [#22](https://github.com/RustForWeb/shadcn-ui/issues/22) | ❌ [#71](https://github.com/RustForWeb/shadcn-ui/issues/71)   | ❌ [#120](https://github.com/RustForWeb/shadcn-ui/issues/120) |
| Chart           | ❌ [#23](https://github.com/RustForWeb/shadcn-ui/issues/23) | ❌ [#72](https://github.com/RustForWeb/shadcn-ui/issues/72)   | ❌ [#121](https://github.com/RustForWeb/shadcn-ui/issues/121) |
| Checkbox        | ❌ [#24](https://github.com/RustForWeb/shadcn-ui/issues/24) | ❌ [#73](https://github.com/RustForWeb/shadcn-ui/issues/73)   | ❌ [#122](https://github.com/RustForWeb/shadcn-ui/issues/122) |
| Collapsible     | ❌ [#25](https://github.com/RustForWeb/shadcn-ui/issues/25) | ❌ [#74](https://github.com/RustForWeb/shadcn-ui/issues/74)   | ❌ [#123](https://github.com/RustForWeb/shadcn-ui/issues/123) |
| Combobox        | ❌ [#26](https://github.com/RustForWeb/shadcn-ui/issues/26) | ❌ [#75](https://github.com/RustForWeb/shadcn-ui/issues/75)   | ❌ [#124](https://github.com/RustForWeb/shadcn-ui/issues/124) |
| Command         | ❌ [#27](https://github.com/RustForWeb/shadcn-ui/issues/27) | ❌ [#76](https://github.com/RustForWeb/shadcn-ui/issues/76)   | ❌ [#125](https://github.com/RustForWeb/shadcn-ui/issues/125) |
| Context Menu    | ❌ [#28](https://github.com/RustForWeb/shadcn-ui/issues/28) | ❌ [#77](https://github.com/RustForWeb/shadcn-ui/issues/77)   | ❌ [#126](https://github.com/RustForWeb/shadcn-ui/issues/126) |
| Data Table      | ❌ [#29](https://github.com/RustForWeb/shadcn-ui/issues/29) | ❌ [#78](https://github.com/RustForWeb/shadcn-ui/issues/78)   | ❌ [#127](https://github.com/RustForWeb/shadcn-ui/issues/127) |
| Date Picker     | ❌ [#30](https://github.com/RustForWeb/shadcn-ui/issues/30) | ❌ [#79](https://github.com/RustForWeb/shadcn-ui/issues/79)   | ❌ [#128](https://github.com/RustForWeb/shadcn-ui/issues/128) |
| Dialog          | ❌ [#31](https://github.com/RustForWeb/shadcn-ui/issues/31) | ❌ [#80](https://github.com/RustForWeb/shadcn-ui/issues/80)   | ❌ [#129](https://github.com/RustForWeb/shadcn-ui/issues/129) |
| Drawer          | ❌ [#32](https://github.com/RustForWeb/shadcn-ui/issues/32) | ❌ [#81](https://github.com/RustForWeb/shadcn-ui/issues/81)   | ❌ [#130](https://github.com/RustForWeb/shadcn-ui/issues/130) |
| Dropdown Menu   | ❌ [#33](https://github.com/RustForWeb/shadcn-ui/issues/33) | ❌ [#82](https://github.com/RustForWeb/shadcn-ui/issues/82)   | ❌ [#131](https://github.com/RustForWeb/shadcn-ui/issues/131) |
| Form            | ❌ [#34](https://github.com/RustForWeb/shadcn-ui/issues/34) | ❌ [#83](https://github.com/RustForWeb/shadcn-ui/issues/83)   | ❌ [#132](https://github.com/RustForWeb/shadcn-ui/issues/132) |
| Hover Card      | ❌ [#35](https://github.com/RustForWeb/shadcn-ui/issues/35) | ❌ [#84](https://github.com/RustForWeb/shadcn-ui/issues/84)   | ❌ [#133](https://github.com/RustForWeb/shadcn-ui/issues/133) |
| Input           | ❌ [#36](https://github.com/RustForWeb/shadcn-ui/issues/36) | ❌ [#85](https://github.com/RustForWeb/shadcn-ui/issues/85)   | ❌ [#134](https://github.com/RustForWeb/shadcn-ui/issues/134) |
| Input OTP       | ❌ [#37](https://github.com/RustForWeb/shadcn-ui/issues/37) | ❌ [#86](https://github.com/RustForWeb/shadcn-ui/issues/86)   | ❌ [#135](https://github.com/RustForWeb/shadcn-ui/issues/135) |
| Label           | ❌ [#38](https://github.com/RustForWeb/shadcn-ui/issues/38) | ❌ [#87](https://github.com/RustForWeb/shadcn-ui/issues/87)   | ❌ [#136](https://github.com/RustForWeb/shadcn-ui/issues/136) |
| Menubar         | ❌ [#39](https://github.com/RustForWeb/shadcn-ui/issues/39) | ❌ [#88](https://github.com/RustForWeb/shadcn-ui/issues/88)   | ❌ [#137](https://github.com/RustForWeb/shadcn-ui/issues/137) |
| Navigation Menu | ❌ [#40](https://github.com/RustForWeb/shadcn-ui/issues/40) | ❌ [#89](https://github.com/RustForWeb/shadcn-ui/issues/89)   | ❌ [#138](https://github.com/RustForWeb/shadcn-ui/issues/138) |
| Pagination      | ❌ [#41](https://github.com/RustForWeb/shadcn-ui/issues/41) | ❌ [#90](https://github.com/RustForWeb/shadcn-ui/issues/90)   | 🟦 [#139](https://github.com/RustForWeb/shadcn-ui/issues/139) |
| Popover         | ❌ [#42](https://github.com/RustForWeb/shadcn-ui/issues/42) | ❌ [#91](https://github.com/RustForWeb/shadcn-ui/issues/91)   | ❌ [#140](https://github.com/RustForWeb/shadcn-ui/issues/140) |
| Progress        | ❌ [#43](https://github.com/RustForWeb/shadcn-ui/issues/43) | ❌ [#92](https://github.com/RustForWeb/shadcn-ui/issues/92)   | ❌ [#141](https://github.com/RustForWeb/shadcn-ui/issues/141) |
| Radio Group     | ❌ [#44](https://github.com/RustForWeb/shadcn-ui/issues/44) | ❌ [#93](https://github.com/RustForWeb/shadcn-ui/issues/93)   | ❌ [#142](https://github.com/RustForWeb/shadcn-ui/issues/142) |
| Resizable       | ❌ [#45](https://github.com/RustForWeb/shadcn-ui/issues/45) | ❌ [#94](https://github.com/RustForWeb/shadcn-ui/issues/94)   | ❌ [#143](https://github.com/RustForWeb/shadcn-ui/issues/143) |
| Scroll Area     | ❌ [#46](https://github.com/RustForWeb/shadcn-ui/issues/46) | ❌ [#95](https://github.com/RustForWeb/shadcn-ui/issues/95)   | ❌ [#144](https://github.com/RustForWeb/shadcn-ui/issues/144) |
| Select          | ❌ [#47](https://github.com/RustForWeb/shadcn-ui/issues/47) | ❌ [#96](https://github.com/RustForWeb/shadcn-ui/issues/96)   | ❌ [#145](https://github.com/RustForWeb/shadcn-ui/issues/145) |
| Separator       | ❌ [#48](https://github.com/RustForWeb/shadcn-ui/issues/48) | ❌ [#97](https://github.com/RustForWeb/shadcn-ui/issues/97)   | ❌ [#146](https://github.com/RustForWeb/shadcn-ui/issues/146) |
| Sheet           | ❌ [#49](https://github.com/RustForWeb/shadcn-ui/issues/49) | ❌ [#98](https://github.com/RustForWeb/shadcn-ui/issues/98)   | ❌ [#147](https://github.com/RustForWeb/shadcn-ui/issues/147) |
| Skeleton        | ❌ [#50](https://github.com/RustForWeb/shadcn-ui/issues/50) | ❌ [#99](https://github.com/RustForWeb/shadcn-ui/issues/99)   | 🟦 [#148](https://github.com/RustForWeb/shadcn-ui/issues/148) |
| Slider          | ❌ [#51](https://github.com/RustForWeb/shadcn-ui/issues/51) | ❌ [#100](https://github.com/RustForWeb/shadcn-ui/issues/100) | ❌ [#149](https://github.com/RustForWeb/shadcn-ui/issues/149) |
| Sonner          | ❌ [#52](https://github.com/RustForWeb/shadcn-ui/issues/52) | ❌ [#101](https://github.com/RustForWeb/shadcn-ui/issues/101) | ❌ [#150](https://github.com/RustForWeb/shadcn-ui/issues/150) |
| Switch          | ❌ [#53](https://github.com/RustForWeb/shadcn-ui/issues/53) | ❌ [#102](https://github.com/RustForWeb/shadcn-ui/issues/102) | ❌ [#151](https://github.com/RustForWeb/shadcn-ui/issues/151) |
| Table           | ❌ [#54](https://github.com/RustForWeb/shadcn-ui/issues/54) | ❌ [#103](https://github.com/RustForWeb/shadcn-ui/issues/103) | ❌ [#152](https://github.com/RustForWeb/shadcn-ui/issues/152) |
| Tabs            | ❌ [#55](https://github.com/RustForWeb/shadcn-ui/issues/55) | ❌ [#104](https://github.com/RustForWeb/shadcn-ui/issues/104) | ❌ [#153](https://github.com/RustForWeb/shadcn-ui/issues/153) |
| Textarea        | ❌ [#56](https://github.com/RustForWeb/shadcn-ui/issues/56) | ❌ [#105](https://github.com/RustForWeb/shadcn-ui/issues/105) | ❌ [#154](https://github.com/RustForWeb/shadcn-ui/issues/154) |
| Toast           | ❌ [#57](https://github.com/RustForWeb/shadcn-ui/issues/57) | ❌ [#106](https://github.com/RustForWeb/shadcn-ui/issues/106) | ❌ [#155](https://github.com/RustForWeb/shadcn-ui/issues/155) |
| Toggle          | ❌ [#58](https://github.com/RustForWeb/shadcn-ui/issues/58) | ❌ [#107](https://github.com/RustForWeb/shadcn-ui/issues/107) | ❌ [#156](https://github.com/RustForWeb/shadcn-ui/issues/156) |
| Toggle Group    | ❌ [#59](https://github.com/RustForWeb/shadcn-ui/issues/59) | ❌ [#108](https://github.com/RustForWeb/shadcn-ui/issues/108) | ❌ [#157](https://github.com/RustForWeb/shadcn-ui/issues/157) |
| Tooltip         | ❌ [#60](https://github.com/RustForWeb/shadcn-ui/issues/60) | ❌ [#109](https://github.com/RustForWeb/shadcn-ui/issues/109) | ❌ [#158](https://github.com/RustForWeb/shadcn-ui/issues/158) |
| **Total**       | 0 / 49                                                      | 0 / 49                                                        | 4 / 49                                                        |

## License

This project is available under the [MIT license](https://github.com/RustForWeb/shadcn-ui/blob/main/LICENSE.md).

## Rust For Web

The Rust shadcn/ui project is part of the [Rust For Web](https://github.com/RustForWeb).

[Rust For Web](https://github.com/RustForWeb) creates and ports web UI libraries for Rust. All projects are free and open source.
