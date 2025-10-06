import { Button, Header, Input, Selection, TextInput } from "@/components";
import { UseSettingsReturn } from "@/types";
import curl2Json, { ResultJSON } from "@bany/curl-to-json";
import { KeyIcon, TrashIcon } from "lucide-react";
import { useEffect, useState } from "react";

const SENSITIVE_VARIABLE_KEYS = [
  "api_key",
  "api_token",
  "access_token",
  "auth_token",
  "bearer_token",
  "client_secret",
  "secret",
  "token",
  "license_key",
  "password",
];

export const Providers = ({
  allAiProviders,
  selectedAIProvider,
  onSetSelectedAIProvider,
  variables,
}: UseSettingsReturn) => {
  const [localSelectedProvider, setLocalSelectedProvider] =
    useState<ResultJSON | null>(null);

  useEffect(() => {
    if (selectedAIProvider?.provider) {
      const provider = allAiProviders?.find(
        (p) => p?.id === selectedAIProvider?.provider
      );
      if (provider) {
        const json = curl2Json(provider?.curl);
        setLocalSelectedProvider(json as ResultJSON);
      }
    }
  }, [selectedAIProvider?.provider]);

  const findSensitiveVariable = () => {
    return variables?.find((variable) =>
      SENSITIVE_VARIABLE_KEYS.includes(variable?.key || "")
    );
  };

  const sensitiveVariable = findSensitiveVariable();

  const getSensitiveValue = () => {
    if (!sensitiveVariable || !selectedAIProvider?.variables) return "";
    return selectedAIProvider?.variables?.[sensitiveVariable.key] || "";
  };

  const isSensitiveValueEmpty = () => {
    return !getSensitiveValue().trim();
  };

  return (
    <div className="space-y-3">
      <div className="space-y-2">
        <Header
          title="Select AI Provider"
          description="Select your preferred AI service provider or custom providers to get started."
        />
        <Selection
          selected={selectedAIProvider?.provider}
          options={allAiProviders?.map((provider) => {
            const json = curl2Json(provider?.curl);
            return {
              label: provider?.isCustom
                ? json?.url || "Custom Provider"
                : provider?.id || "Custom Provider",
              value: provider?.id || "Custom Provider",
              isCustom: provider?.isCustom,
            };
          })}
          placeholder="Choose your AI provider"
          onChange={(value) => {
            onSetSelectedAIProvider({
              provider: value,
              variables: {},
            });
          }}
        />
      </div>

      {localSelectedProvider ? (
        <Header
          title={`Method: ${
            localSelectedProvider?.method || "Invalid"
          }, Endpoint: ${localSelectedProvider?.url || "Invalid"}`}
          description={`If you want to use different url or method, you can always create a custom provider.`}
        />
      ) : null}

      {sensitiveVariable ? (
        <div className="space-y-2">
          <Header
            title={sensitiveVariable.value || "Secret"}
            description={`Enter your ${
              allAiProviders?.find(
                (p) => p?.id === selectedAIProvider?.provider
              )?.isCustom
                ? "Custom Provider"
                : selectedAIProvider?.provider
            } API key to authenticate and access AI models. Your key is stored locally and never shared.`}
          />

          <div className="space-y-2">
            <div className="flex gap-2">
              <Input
                type="password"
                placeholder="**********"
                value={getSensitiveValue()}
                onChange={(value) => {
                  if (!sensitiveVariable || !selectedAIProvider) return;

                  onSetSelectedAIProvider({
                    ...selectedAIProvider,
                    variables: {
                      ...selectedAIProvider.variables,
                      [sensitiveVariable.key]:
                        typeof value === "string" ? value : value.target.value,
                    },
                  });
                }}
                onKeyDown={(e) => {
                  if (!sensitiveVariable || !selectedAIProvider) return;

                  onSetSelectedAIProvider({
                    ...selectedAIProvider,
                    variables: {
                      ...selectedAIProvider.variables,
                      [sensitiveVariable.key]: (e.target as HTMLInputElement).value,
                    },
                  });
                }}
                disabled={false}
                className="flex-1 h-11 border-1 border-input/50 focus:border-primary/50 transition-colors"
              />
              {isSensitiveValueEmpty() ? (
                <Button
                  onClick={() => {
                    if (!sensitiveVariable || !selectedAIProvider || isSensitiveValueEmpty())
                      return;

                    onSetSelectedAIProvider({
                      ...selectedAIProvider,
                      variables: {
                        ...selectedAIProvider.variables,
                        [sensitiveVariable.key]: getSensitiveValue(),
                      },
                    });
                  }}
                  disabled={isSensitiveValueEmpty()}
                  size="icon"
                  className="shrink-0 h-11 w-11"
                  title="Submit API Key"
                >
                  <KeyIcon className="h-4 w-4" />
                </Button>
              ) : (
                <Button
                  onClick={() => {
                    if (!sensitiveVariable || !selectedAIProvider) return;

                    onSetSelectedAIProvider({
                      ...selectedAIProvider,
                      variables: {
                        ...selectedAIProvider.variables,
                        [sensitiveVariable.key]: "",
                      },
                    });
                  }}
                  size="icon"
                  variant="destructive"
                  className="shrink-0 h-11 w-11"
                  title="Remove API Key"
                >
                  <TrashIcon className="h-4 w-4" />
                </Button>
              )}
            </div>
          </div>
        </div>
      ) : null}

      <div className="space-y-4 mt-2">
        {variables
          .filter((variable) => variable?.key !== sensitiveVariable?.key)
          .map((variable) => {
            const getVariableValue = () => {
              if (!variable?.key || !selectedAIProvider?.variables) return "";
              return selectedAIProvider.variables[variable.key] || "";
            };

            const isSensitiveField = SENSITIVE_VARIABLE_KEYS.includes(
              variable?.key || ""
            );

            const placeholder = `Enter ${
              allAiProviders?.find((p) => p?.id === selectedAIProvider?.provider)
                ?.isCustom
                ? "Custom Provider"
                : selectedAIProvider?.provider
            } ${variable?.key?.replace(/_/g, " ") || "value"}`;

            return (
              <div className="space-y-1" key={variable?.key}>
                <Header
                  title={variable?.value || ""}
                  description={`add your preferred ${variable?.key?.replace(
                    /_/g,
                    " "
                  )} for ${
                    allAiProviders?.find(
                      (p) => p?.id === selectedAIProvider?.provider
                    )?.isCustom
                      ? "Custom Provider"
                      : selectedAIProvider?.provider
                  }`}
                />
                {isSensitiveField ? (
                  <Input
                    type="password"
                    placeholder={placeholder}
                    value={getVariableValue()}
                    onChange={(event) => {
                      if (!variable?.key || !selectedAIProvider) return;

                      onSetSelectedAIProvider({
                        ...selectedAIProvider,
                        variables: {
                          ...selectedAIProvider.variables,
                          [variable.key]: event.target.value,
                        },
                      });
                    }}
                    className="h-11 border-1 border-input/50 focus:border-primary/50 transition-colors"
                  />
                ) : (
                  <TextInput
                    placeholder={placeholder}
                    value={getVariableValue()}
                    onChange={(value) => {
                      if (!variable?.key || !selectedAIProvider) return;

                      onSetSelectedAIProvider({
                        ...selectedAIProvider,
                        variables: {
                          ...selectedAIProvider.variables,
                          [variable.key]: value,
                        },
                      });
                    }}
                  />
                )}
              </div>
            );
          })}
      </div>
    </div>
  );
};
