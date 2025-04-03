import { Navigate, type RouteObject, createMemoryRouter } from "react-router";
import { WindowLayout } from "@/layouts/window";
import { HomePage } from "@/pages/home";
import { LocalizationsPage } from "@/pages/localizations";
import { SettingsPage } from "@/pages/settings";
import { AboutPage } from "@/pages/about";
import { LocalizationPage } from "@/pages/localization";
import { rootStore } from "@/stores";
import { MainLayout } from "./layouts/main";

export const routes: RouteObject[] = [
  {
    element: <WindowLayout />,
    children: [
      {
        element: <MainLayout />,
        loader: async () => {
          await rootStore.settings.loadSettings();
        },
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
        ]
      }
    ],
  },
];

export const router = createMemoryRouter(routes);
