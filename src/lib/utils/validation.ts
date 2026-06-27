const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

export function validateEmail(value: string): string | null {
	if (!value) return null; // let required handle empty
	if (!emailRegex.test(value)) {
		return 'Please enter a valid email address';
	}
	return null;
}

const phoneRegex = /^(\+1\s?)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$/;

export function validatePhone(value: string): string | null {
	if (!value) return null; // let required handle empty
	if (!phoneRegex.test(value.trim())) {
		return 'Please enter a valid phone number (e.g., (203) 555-1234 or 203-555-1234)';
	}
	return null;
}