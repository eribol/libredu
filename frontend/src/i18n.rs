use crate::app;
use std::collections::BTreeMap;
use unic_langid::{langid, LanguageIdentifier};
use zoon::{eprintln, *};

// @TODO generalize and move to Zoon?
// @TODO translation lists compile-time / lazy loading?
// @TODO make it typed?

// ------ types & aliases ------

pub use fluent::{
    fluent_args as translation_args, FluentArgs as TranslationArgs,
    FluentResource as TranslationResource,
};
type TranslationList = fluent::FluentBundle<TranslationResource>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(crate = "serde")]
pub enum Lang {
    En,
    #[default]
    Tr,
}

impl Lang{
    pub fn label(self)->&'static str{
        match self{
            Lang::En => "En",
            Lang::Tr => "Tr"
        }
    }
}

// ------ states ------

#[static_ref]
pub fn lang() -> &'static Mutable<Lang> {
    fn load_lang() -> Option<Lang> {
        local_storage()
            .get(app::LANG_STORAGE_KEY)
            .and_then(|lang| lang.ok())
    }
    fn store_lang(lang: Lang) {
        if let Err(error) = local_storage().insert(app::LANG_STORAGE_KEY, &lang) {
            // @TODO translate and show error
            eprintln!("failed to store lang: {error:#}")
        }
    }
    fn set_html_lang(lang: Lang) {
        // https://www.w3schools.com/tags/ref_language_codes.asp
        let html_lang = match lang {
            Lang::En => "en",
            Lang::Tr => "tr",
        };
        // @TODO move to Zoon
        document()
            .document_element()
            .unwrap_throw()
            .unchecked_into::<web_sys::HtmlElement>()
            .set_lang(html_lang);
    }

    let mutable_lang = Mutable::new(load_lang().unwrap_or_default());
    Task::start(mutable_lang.signal().for_each_sync(|lang| {
        store_lang(lang);
        set_html_lang(lang);
    }));
    mutable_lang
}

#[static_ref]
fn translation_lists() -> &'static SendWrapper<BTreeMap<Lang, TranslationList>> {
    fn create_translation_list(
        lang_id: LanguageIdentifier,
        list_file_content: &'static str,
    ) -> TranslationList {
        let resource = TranslationResource::try_new(list_file_content.to_owned()).unwrap_or_else(
            |(_resource, errors)| panic!("failed to parse translation list: {errors:?}"),
        );

        let mut bundle = TranslationList::new(vec![lang_id]);
        bundle
            .add_resource(resource)
            .expect_throw("failed to add translation list");
        bundle
    }
    macro_rules! create_translation_list {
        ( $lang_id:literal ) => {
            create_translation_list(
                langid!($lang_id),
                include_str!(concat!("../languages/", $lang_id, ".ftl")),
            )
        };
    }
    SendWrapper::new(BTreeMap::from_iter([
        (Lang::En, create_translation_list!("en-US")),
        (Lang::Tr, create_translation_list!("tr-TR")),
    ]))
}

// ------ translate functions ------

pub fn translate(
    key: impl IntoCowStr<'static>,
    args: Option<TranslationArgs>,
) -> impl Signal<Item = String> + '_ {
    let key = key.into_cow_str();
    lang()
        .signal()
        .map(move |lang| translate_with_lang(lang, &key, args.as_ref()))
}


pub fn translate_static(key: impl IntoCowStr<'static>, args: Option<TranslationArgs>) -> String {
    translate_with_lang(lang().get(), &key.into_cow_str(), args.as_ref())
}


fn translate_with_lang(lang: Lang, key: &str, args: Option<&TranslationArgs>) -> String {
    let translation_list = translation_lists()
        .get(&lang)
        .unwrap_or_else(|| panic!("translation list for {lang:?} not found"));

    let translation_object = translation_list
        .get_message(key)
        .unwrap_or_else(|| panic!("translation key \"{key}\" not found"));

    let pattern = translation_object
        .value()
        .expect_throw("failed to get translation object value");

    let mut errors = Vec::new();
    let translation = translation_list
        .format_pattern(pattern, args, &mut errors)
        .to_string();

    if not(errors.is_empty()) {
        panic!("format translation errors: {errors:?}");
    }
    translation
}

// ------ macros ------

macro_rules! t {
    ($key:expr) => {
        $crate::i18n::translate($key, None)
    };
    ($key:expr, $($arg_name:ident = $arg_value:expr),*) => {
        $crate::i18n::translate($key, Some(crate::i18n::translation_args![
            $(
                stringify!($arg_name) => $arg_value
            )*
        ]))
    };
}
pub(crate) use t;


macro_rules! t_s {
    ($key:expr) => {
        $crate::i18n::translate_static($key, None)
    };
    ($key:expr, $($arg_name:ident = $arg_value:expr),*) => {
        $crate::i18n::translate_static($key, Some(crate::i18n::translation_args![
            $(
                stringify!($arg_name) => $arg_value
            )*
        ]))
    };
}
pub(crate) use t_s;


pub fn change_locale(){
    lang().update_mut(|l|
        match l{
        Lang::En => *l = Lang::Tr,
        Lang::Tr => *l = Lang::En
    }
    )
    //lang().set(Lang::En)
}