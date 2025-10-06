import { Header, Label, Switch } from "@/components";
import { useApp } from "@/contexts";
import { useMemo } from "react";

interface ScreenProtectionToggleProps {
  className?: string;
}

export const ScreenProtectionToggle = ({
  className,
}: ScreenProtectionToggleProps) => {
  const { screenProtectionEnabled, setScreenProtectionEnabled } = useApp();

  const isWindows = useMemo(() => {
    if (typeof navigator === "undefined") {
      return false;
    }
    return navigator.userAgent.toLowerCase().includes("windows");
  }, []);

  const handleSwitchChange = async (checked: boolean) => {
    await setScreenProtectionEnabled(checked);
  };

  return (
    <div className={`space-y-2 ${className ?? ""}`}>
      <Header
        title="Screen Capture Protection"
        description="Keep Pluely hidden from screen sharing, screenshots, and recording tools. Recommended for interviews and exams."
        isMainTitle
      />
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <Label className="text-sm font-medium">
            {screenProtectionEnabled ? "Protection enabled" : "Protection disabled"}
          </Label>
          <p className="text-xs text-muted-foreground max-w-sm">
            {isWindows
              ? "When enabled, Pluely stays invisible to screen capture tools on Windows 10 (build 19041+) and Windows 11."
              : "Screen capture protection is only available on Windows."}
          </p>
        </div>
        <Switch
          checked={screenProtectionEnabled}
          onCheckedChange={handleSwitchChange}
          disabled={!isWindows}
          title={
            screenProtectionEnabled
              ? "Disable screen capture protection"
              : "Enable screen capture protection"
          }
          aria-label="Toggle screen capture protection"
        />
      </div>
    </div>
  );
};
