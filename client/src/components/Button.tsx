type Props = {
  onClick: () => void;
  disabled: boolean;
  children: React.ReactNode;
};

export default function Button({ onClick, disabled, children }: Props) {
  return (
    <div className="p-3 pt-5">
      <button
        onClick={onClick}
        disabled={disabled}
        className={`bg-blue-500 text-white font-bold py-2 px-4 rounded ${disabled ? "opacity-50 cursor-not-allowed" : ""}`}
      >
        {children}
      </button>
    </div>
  );
}
