import * as variables from "@spinframework/spin-variables";

export interface Config {
    ollamaUrl: string;
    model: string;
    temperature: number;
}

export function loadConfig(): Config | undefined {
    const ollamaUrl = variables.get("ollama_api_url");
    const model = variables.get("model");
    const temperature = variables.get("temperature");

    if (!ollamaUrl || !model || !temperature) {
        console.log("Missing required configuration variables");
        return undefined;
    }

    let temp = parseFloat(temperature!);
    if (isNaN(temp) || temp < 0 || temp > 1) {
        console.log("Temperature must be a number between 0 and 1");
        return undefined;
    }

    return {
        ollamaUrl,
        model,
        temperature: temp
    } as Config;
}