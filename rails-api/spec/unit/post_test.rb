require 'rails_helper'

RSpec.describe Post, type: :model do
  it 'should build properly' do
    expect(FactoryGirl.build(:post)).to be_valid
  end

  it 'should create properly' do
    expect(FactoryGirl.create(:post)).to be_valid
  end
end
