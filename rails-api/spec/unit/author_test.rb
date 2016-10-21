require 'rails_helper'

RSpec.describe Author, type: :model do
  it 'should build properly' do
    expect(FactoryGirl.build(:author)).to be_valid
  end

  it 'should create properly' do
    expect(FactoryGirl.create(:author)).to be_valid
  end
end
