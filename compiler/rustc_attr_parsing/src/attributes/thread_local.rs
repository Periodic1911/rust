use rustc_attr_data_structures::AttributeKind;
use rustc_feature::{AttributeTemplate, template};
use rustc_span::{Symbol, sym};

use crate::attributes::{AttributeOrder, OnDuplicate, SingleAttributeParser};
use crate::context::{AcceptContext, Stage};
use crate::parser::ArgParser;

pub(crate) struct ThreadLocalParser;
impl<S: Stage> SingleAttributeParser<S> for ThreadLocalParser {
    const PATH: &[Symbol] = &[sym::thread_local];
    const ATTRIBUTE_ORDER: AttributeOrder = AttributeOrder::KeepOutermost;
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const TEMPLATE: AttributeTemplate = template!(Word); // Anything, really

    fn convert(cx: &mut AcceptContext<'_, '_, S>, _: &ArgParser<'_>) -> Option<AttributeKind> {
        Some(AttributeKind::ThreadLocal(cx.attr_span))
    }
}
