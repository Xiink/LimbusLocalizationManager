import { toast, ToastContentProps } from "react-toastify";
import styles from "./toast.module.css";
import { X } from "lucide-react";
import { cn } from "@/utils";

function Toast({ closeToast, toastProps }: ToastContentProps) {
  const data = toastProps.data as { message: string };

  return (
    <div className={cn(styles.toast, toastProps.type === "error" && styles.error)}>
      <h1>{data.message}</h1>
      <button onClick={closeToast}>
        <X className="w-4 h-4" />
      </button>
    </div>
  );
}

export function toastError(message: string) {
  toast(Toast, {
    className: cn(styles.container, styles.error),
    data: {
      message,
    },
  });
}

export function toastSuccess(message: string) {
  toast(Toast, {
    className: styles.container,
    type: "success",
    data: {
      message,
    },
  });
}

export function toastInfo(message: string) {
  toast(Toast, {
    className: styles.container,
    type: "info",
    data: {
      message,
    },
  });
} 