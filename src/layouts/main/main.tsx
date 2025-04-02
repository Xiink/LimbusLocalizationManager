import { Outlet } from "react-router";
import Navbar from "@/components/navbar/navbar";
import styles from "./main.module.css";
import Header from "@/components/header/header";
import { ToastContainer } from "react-toastify";

function MainLayout() {
  return (
    <div className={styles.main}>
      <Header />
      <div className={styles.container}>
        <Navbar />
        <div className={styles.page}>
          <Outlet />
        </div>
      </div>
      <ToastContainer position="bottom-right" hideProgressBar />
    </div>
  );
}

export default MainLayout;
