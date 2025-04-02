import { type RouteObject, createMemoryRouter } from "react-router";
import { MainLayout } from "@/layouts/main";
import { HomePage } from "@/pages/home";
import { LocalizationsPage } from "@/pages/localizations";
import { SettingsPage } from "@/pages/settings";
import { AboutPage } from "@/pages/about";
import { LocalizationPage } from "@/pages/localization";
import { rootStore } from "@/stores";
export const routes: RouteObject[] = [
  {
    element: <MainLayout />,

    // TODO: Add error handling
    loader: async () => {
      await rootStore.settings.loadSettings();
    },

    children: [
      {
        path: "/",
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
];

export const router = createMemoryRouter(routes);
