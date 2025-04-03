import { useRouteError } from "react-router";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";

function ErrorBoundary() {
  const { t } = useTranslation();
  const error = useRouteError();
  
  useEffect(() => {
    console.error(error);
  }, [error]);

  return (
    <div className="flex-1 flex flex-col items-center justify-center text-center">
      <h1 className="text-danger text-lg">{t("error.somethingWentWrong")}</h1>
      {error instanceof Error && (
        <p className="text-danger">{error.message}</p>
      )}
    </div>
  );
}

export default ErrorBoundary;
