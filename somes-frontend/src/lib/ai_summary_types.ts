export type DbAiSummary = {
  id: number
  full_summary: AiSummary
  short_title: string
  short_summary: string
  detailed_summary: string
  very_detailed_summary: string
  complexity_scope_of_proposal: string
  model_used: string
  version: string
  generated_at: string
}

export type EnforcementDates = {
  enforcement_start_date: string
  start_notes?: string | null
  enforcement_end_date?: string | null
  end_notes?: string | null
}

export type Keypoint = {
  point: string
  enforcement_start_end?: EnforcementDates | null
  paragraph_references: EuroLawReference[]
}

export enum ProposalComplexityScope {
  Highest = "Highest",
  High = "High",
  Medium = "Medium",
  Low = "Low",
  Lowest = "Lowest",
}

export enum Tone {
  Neutral = "Neutral",
  Urgent = "Urgent",
  Aggressive = "Aggressive",
  Optimistic = "Optimistic",
  Bureaucratic = "Bureaucratic",
}

export type CriticalAnalysis = {
  arguments_for: string[]
  arguments_against: string[]
  tone: Tone
}

export type TermDefinition = {
  term: string
  simple_definition: string
}

export type Glossary = {
  difficult_terms: TermDefinition[]
}

export enum EconomicImpactLevel {
  HighCost = "HighCost",
  ModerateCost = "ModerateCost",
  LowCost = "LowCost",
  Neutral = "Neutral",
  RevenueGenerating = "RevenueGenerating",
  Unclear = "Unclear",
}

export type FiscalAnalysis = {
  estimated_cost_per_year_in_million?: number | null
  estimated_cost_per_month_in_million?: number | null
  economic_burden: EconomicImpactLevel
}

export type AiSummary = {
  short_title: string
  short_summary: string
  detailed_summary: string
  key_points: Keypoint[]
  general_enforcement_start_end?: EnforcementDates | null
  topics: string[]
  general_political_questions: string[]
  political_compass_questions: string[]
  complexity_scope_of_proposal: ProposalComplexityScope
  critical_analysis: CriticalAnalysis
  glossary: Glossary
  fiscal_analysis: FiscalAnalysis
}

export type LawPart =
  | { kind: "Article"; value: number }
  | { kind: "Paragraph"; value: number }
  | { kind: "Literal"; value: string }
  | { kind: "Subsection"; value: number }
  | { kind: "Point"; value: string }

export type EuroLawReference = {
  full_unparsed_reference: string
  parsed_reference_parts: LawPart[]
}

export type SummarizeOutput = {
  summary?: AiSummary | null
  abort_reason: AbortReason
}

export enum AbortReason {
  NoDocumentsProvidedInContext = "NoDocumentsProvidedInContext",
  None = "None",
}
