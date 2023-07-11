use leptos::*;
use cfg_if::cfg_if;
use crate::model::conversation::Conversation;


#[server(Converse "/api")]
pub async fn converse(cx: Scope, conversation: Conversation) -> Result<(), ServerFnError>{
    use llm::{ models::Llama, KnownModel };
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    }).await.unwrap();

    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant";
    let mut history = format!(
        "{character_name}: Hello - How may I help you?\n\
        {user_name}: How can you help me?\n\
        {character_name}: I am here to provide answers to your questions\n"
    );

    for message in conversation.messages.into_iter() {
        let msg = message.text;
        let curr_user = if message.from_user { character_name } else { user_name };
        let curr_line = format!("{curr_user}: {msg}\n");

        history.push(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());

    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}:")
                .as_str().into(),
            parameters: Some(&llm::InferenceParameters::default()),
            play_back_previous_tokens: false,
            maximum_token_count: None
        },
        &mut Default::default(),
        inference_callback(
            String::from(user_name),
            &mut buf,
            &mut res
        )
    ).unwrap_or_else(|e| panic!("{e}"));

    Ok(())
}


cfg_if! {
    if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(&t);
                    } else {
                        out_str.push_str(&reverse_buf);
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}