// Generic domain types for the crackedstack starter template.
// Replace or extend these with your own application types.

export interface Item {
	id: string;
	title: string;
	description?: string;
	created_by: string;
	created_at: string;
	updated_at: string;
}

export interface ItemInput {
	title: string;
	description?: string;
}

export interface ProfileData {
	email: string;
	name: string;
	role: 'user' | 'admin';
}
