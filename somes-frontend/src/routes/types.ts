export type PlatformItemType = 'vote' | 'proposal' | 'decree';

export interface PlatformItem {
    id: number;
    type: PlatformItemType;
    title: string;
    date: string;
    status?: 'accepted' | 'rejected' | 'pending';
}