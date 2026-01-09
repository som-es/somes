export type PlatformItemType = 'vote' | 'proposal' | 'decree';

export interface PlatformItem {
    id: number;
    path?: string;
    type: PlatformItemType;
    title: string;
    date: string;
    status?: 'accepted' | 'rejected' | 'pending';
}