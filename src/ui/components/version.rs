use relm4::prelude::*;

use adw::prelude::*;

use std::path::PathBuf;

pub enum VersionState {
    Downloaded,
    Downloading(u64, u64),
    Unpacking(u64, u64),
    NotDownloaded
}

pub struct ComponentVersion {
    pub name: String,
    pub title: String,
    pub recommended: bool,

    pub download_uri: String,
    pub download_folder: PathBuf,

    pub show_recommended_only: bool,
    pub state: VersionState
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool)
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentVersion {
    type Init = (super::ComponentsListVersion, PathBuf);
    type Input = AppMsg;
    type Output = ();

    view! {
        row = adw::ActionRow {
            set_title: &model.title,

            #[watch]
            set_visible: !model.show_recommended_only || model.recommended,

            add_suffix = &gtk::Button {
                #[watch]
                set_icon_name: match model.state {
                    VersionState::NotDownloaded => "document-save-symbolic",

                    // In other states it will be downloaded or hidden
                    _ => "user-trash-symbolic"
                },

                add_css_class: "flat",
                set_valign: gtk::Align::Center
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = ComponentVersion {
            name: init.0.name,
            title: init.0.title,
            recommended: init.0.recommended,

            download_uri: init.0.uri,
            download_folder: init.1,

            show_recommended_only: true,
            state: VersionState::NotDownloaded
        };

        // Set default component state
        model.state = if model.download_folder.join(&model.name).exists() {
            VersionState::Downloaded
        } else {
            VersionState::NotDownloaded
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called component version [{}] event: {:?}", self.title, msg);

        match msg {
            AppMsg::ShowRecommendedOnly(state) => self.show_recommended_only = state
        }
    }
}
