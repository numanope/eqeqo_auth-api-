// --- Common ---

export interface SocialLink {
  name: 'instagram' | 'twitter' | 'facebook' | 'linkedin' | 'github';
  url: string;
}

// --- Home Page ---

export interface HomeDTO {
  title: string;
  subtitle: string;
  hero_image_url: string;
  blocks_md: string[];
  footer_text_md: string;
  social_links: SocialLink[];
}

// --- Products ---

export interface ProductDTO {
  id: string;
  name: string;
  price: number;
  stock: number;
  image_url: string;
  tags: string[];
}

export interface ProductsDTO {
  items: ProductDTO[];
  tags: string[];
}

// --- Filters ---

export interface FilterOption {
  id: string;
  label: string;
  tag: string;
}

export interface Filter {
  id: string;
  label: string;
  options: FilterOption[];
}

export interface FiltersDTO {
  items: Filter[];
}

// --- Contact Page ---

export interface ContactDTO {
  content_md: string;
}

// --- API Payloads ---

export type UpdateHomePayload = Omit<HomeDTO, 'social_links'>;
export type UpdateProductPayload = Partial<Omit<ProductDTO, 'id' | 'tags'>>;