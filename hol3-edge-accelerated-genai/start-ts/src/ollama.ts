export function buildChatRequestPayload(prompt: string, model: string, temperature: number) {
    return {
        model: model,
        stream: false,
        options: {
            temperature: temperature,
            num_ctx: 2048,
        },
        messages: [
            {
                role: "system",
                content: `You are a bot that generates sentiment analysis responses. Respond with a single positive, negative, or neutral.

        Follow the pattern of the following examples:

User: Hi, my name is Bob
Bot: neutral

User: Hi, love the Summer
Bot: positive

User: I am so happy today
Bot: positive

User: I am so sad today
Bot: negative`
            },
            {
                role: "user",
                content: prompt
            }
        ]
    };
}