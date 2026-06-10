import { Inter } from "next/font/google";
import { ShieldCheck } from "lucide-react";
import Providers from "./providers";
import "./globals.css";

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" });

export const metadata = {
  title: {
    default: "Certificate Inventory",
    template: "%s · Certificate Inventory",
  },
  description: "TLS certificate inventory and expiry monitoring dashboard.",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en" className={inter.variable}>
      <body className="min-h-screen bg-slate-50 font-sans text-slate-900 antialiased">
        <Providers>
          <header className="border-b border-slate-200 bg-white">
            <div className="mx-auto flex max-w-7xl items-center gap-3 px-4 py-4 sm:px-6 lg:px-8">
              <div className="flex h-8 w-8 items-center justify-center rounded-md bg-blue-600">
                <ShieldCheck className="h-5 w-5 text-white" aria-hidden="true" />
              </div>
              <div>
                <p className="text-sm font-semibold leading-tight text-slate-900">
                  certify-service
                </p>
                <p className="text-xs leading-tight text-slate-500">
                  TLS Certificate Inventory
                </p>
              </div>
            </div>
          </header>
          <main className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
            {children}
          </main>
        </Providers>
      </body>
    </html>
  );
}
