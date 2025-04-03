import { Outlet } from "react-router";
import styles from "./window.module.css";
import Header from "@/components/header/header";
import { ToastContainer } from "react-toastify";

function Window() {
  return (
    <div className={styles.container}>
      <Header />
      <Outlet />
      <ToastContainer position="bottom-right" hideProgressBar />
    </div>
  );
}

export default Window;
