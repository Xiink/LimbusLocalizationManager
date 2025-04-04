import { Navigate, type RouteObject, createMemoryRouter } from "react-router";
import { WindowLayout } from "@/layouts/window";
import { HomePage } from "@/pages/home";
import { LocalizationsPage } from "@/pages/localizations";
import { SettingsPage } from "@/pages/settings";
import { AboutPage } from "@/pages/about";
import { LocalizationPage } from "@/pages/localization";
import { rootStore } from "@/stores";
import { MainLayout } from "./layouts/main";
import Fallback from "@/components/fallback/fallback";
import ErrorBoundary from "@/components/error-boundary/error-boundary";

export const routes: RouteObject[] = [
  {
    element: <WindowLayout />,
    children: [
      {
        element: <MainLayout />,
        errorElement: <ErrorBoundary />,
        loader: async () => {
          await rootStore.state.loadState();
        },
        hydrateFallbackElement: <Fallback />,
        children: [
          {
            index: true,
            element: <Navigate to="/home" />,
          },
          {
            path: "/home",
            element: <HomePage />,
          },
          {
            path: "/localizations",
            element: <LocalizationsPage />,
            children: [
              {
                path: ":id",
                element: <LocalizationPage />,
              },
              {
                index: true,
                element: <LocalizationPage />,
              },
            ],
          },
          {
            path: "/settings",
            element: <SettingsPage />,
          },
          {
            path: "/about",
            element: <AboutPage />,
          },
        ],
      },
    ],
  },
];

export const router = createMemoryRouter(routes);
