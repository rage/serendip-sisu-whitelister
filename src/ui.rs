use iced::{
    widget::{
        button::{self, Button, Status},
        column, container, progress_bar, row, scrollable, text,
    },
    Alignment, Element, Length, Theme,
};

use crate::app::{Message, State, TaskState};

/// Constructs the main application view based on current state
pub fn view(state: &State) -> Element<Message> {
    let content: Element<'_, Message> = match state.task_state {
        TaskState::Idle => build_idle_view(),
        TaskState::Running | TaskState::Completed => build_task_view(state),
    };

    let base_view = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Shrink)
        .center_y(Length::Shrink);

    if state.show_error {
        overlay(base_view, build_error_dialog(state))
    } else if state.show_file_selector {
        overlay(base_view, build_file_selector(state))
    } else if state.show_dialog {
        overlay(base_view, build_completion_dialog())
    } else {
        base_view.into()
    }
}

/// Creates the idle state view with welcome message and controls
fn build_idle_view() -> Element<'static, Message> {
    column![
        text("").size(32),
        text("Serendip Whitelister")
            .size(32)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        text(
            "Welcome to Serendip whitelister.\n\n\
            For documentation on how to use this application, please visit the following link."
        )
        .size(16),
        Button::new("View Documentation").on_press(Message::OpenLink(
            "https://example.com/documentation".into()
        )),
        Button::new("Open Directory").on_press(Message::OpenDirectory),
        Button::new("Start Task").on_press(Message::StartPressed)
    ]
    .spacing(20)
    .align_x(Alignment::Center)
    .into()
}

/// Creates the task execution view with progress and controls
fn build_task_view(state: &State) -> Element<'_, Message> {
    let logs = state.log_buffer.lock().unwrap().join("\n");
    let log_view = scrollable(text(logs)).anchor_bottom();

    column![
        text(if state.task_state == TaskState::Running {
            "Task in progress..."
        } else {
            "Task completed successfully!"
        })
        .size(24),
        progress_bar(0.0..=1.0, state.displayed_progress),
        row![
            Button::new("Open Directory").on_press(Message::OpenDirectory),
            Button::new("Open Current Log File").on_press(Message::OpenCurrentLog),
        ]
        .spacing(10),
        log_view
    ]
    .spacing(20)
    .align_x(Alignment::Center)
    .into()
}

/// Creates an error dialog overlay
fn build_error_dialog(state: &State) -> Element<'_, Message> {
    container(
        column![
            text("Error")
                .size(28)
                .style(|_theme: &Theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.8, 0.2, 0.2)),
                }),
            text(&state.error_message),
            Button::new("OK").on_press(Message::DismissError)
        ]
        .spacing(20)
        .align_x(Alignment::Center)
        .padding(30),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .style(|_| container::Style {
        background: Some(iced::Color::from_rgb(0.2, 0.2, 0.2).into()),
        ..container::Style::default()
    })
    .into()
}

/// Creates a file selector dialog overlay
fn build_file_selector(state: &State) -> Element<'_, Message> {
    let file_list = state
        .csv_files
        .iter()
        .enumerate()
        .map(|(index, (path, created))| {
            let created_time = chrono::DateTime::<chrono::Local>::from(*created)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

            let is_selected = Some(index) == state.selected_csv_index;
            let btn = Button::new(text(format!(
                "{} ({})",
                path.file_name().unwrap_or_default().to_string_lossy(),
                created_time
            )))
            .width(Length::Fill)
            .on_press(Message::FileSelected(index));

            if is_selected {
                btn.style(|_theme: &Theme, _status: Status| button::Style {
                    background: Some(iced::Color::from_rgb(0.2, 0.5, 0.8).into()),
                    ..button::Style::default()
                })
            } else {
                btn
            }
            .into()
        });

    container(
        column![
            text("Select CSV File").size(28),
            column(file_list).spacing(10).width(Length::Fill),
            row![
                Button::new("Cancel")
                    .on_press(Message::CancelFileSelection)
                    .style(|_theme: &Theme, _status: Status| button::Style {
                        background: Some(iced::Color::from_rgb(0.4, 0.4, 0.4).into()),
                        ..button::Style::default()
                    }),
                Button::new("Confirm").on_press(Message::ConfirmFileSelection)
            ]
            .spacing(10)
            .width(Length::Fill)
        ]
        .spacing(20)
        .padding(30)
        .width(Length::Fill)
        .max_width(800),
    )
    .style(|_| container::Style {
        background: Some(iced::Color::from_rgb(0.2, 0.2, 0.2).into()),
        ..container::Style::default()
    })
    .into()
}

/// Creates a completion dialog overlay
fn build_completion_dialog() -> Element<'static, Message> {
    container(
        column![
            text("Task Completed!").size(28),
            Button::new("OK").on_press(Message::DismissDialog)
        ]
        .spacing(20)
        .align_x(Alignment::Center)
        .padding(30),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .style(|_| container::Style {
        background: Some(iced::Color::from_rgb(0.2, 0.2, 0.2).into()),
        ..container::Style::default()
    })
    .into()
}

/// Creates a modal overlay with the given content
pub fn overlay<'a>(
    base: impl Into<Element<'a, Message>>,
    dialog: Element<'a, Message>,
) -> Element<'a, Message> {
    iced::widget::stack![
        base.into(),
        iced::widget::opaque(iced::widget::mouse_area(iced::widget::center(
            iced::widget::opaque(dialog)
        )))
    ]
    .into()
}
